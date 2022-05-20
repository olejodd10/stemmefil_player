use midir::MidiOutputConnection;

use midly::MidiMessage::{
    self,
    NoteOff,
    NoteOn,
    Aftertouch,
    Controller,
    ProgramChange,
    ChannelAftertouch,
    PitchBend,
};

use std::time::{Duration, Instant};
use std::collections::HashSet;
use std::sync::mpsc::Receiver;

use crate::conn::{
    note_on, 
    note_off, 
    silence, 
    silence_all, 
    pan, 
    volume, 
    program_change, 
    controller,
    aftertouch,
    channel_aftertouch,
    pitch_bend,
};

use crate::command::Command::{
    self,
    Play,
    Pause,
    Muted,
    Volume,
    Jump,
    Pan,
};

// Binary search for starting point. Test this.
fn starting_index(indexed_timed_messages: &[(usize,u32,MidiMessage)], time: u32) -> usize {
    let mut low = 0;
    let mut high = indexed_timed_messages.len();
    while high > low + 1 {
        let mid = (low + high)/2;
        if indexed_timed_messages[mid].1 >= time {
            high = mid;
        } else {
            low = mid;
        }
    }
    low
}

pub fn play_real_time(conn: &mut MidiOutputConnection, indexed_timed_messages: &[(usize,u32,MidiMessage)], recv: Receiver<Command>) {
    let mut muted_tracks = HashSet::new();
    let end_time = indexed_timed_messages.last().unwrap().1;
    let mut time = 0;
    let mut index = 0;
    let mut paused = false;
    'outer: loop { 
        let (track_id, new_time, message) = &indexed_timed_messages[index];
        let sleep_duration = Duration::from_micros(new_time.checked_sub(time).unwrap_or(0) as u64);
        let now = Instant::now();
        // Listen for commands
        while paused || now.elapsed() < sleep_duration {
            if let Ok(command) = recv.try_recv() {
                match command {
                    Play => paused = false,
                    Pause => {
                        silence_all(conn);
                        paused = true;
                    },
                    Muted(id, muted) => {
                        if muted {
                            silence(conn, id as u8);
                            muted_tracks.insert(id);
                        } else {
                            muted_tracks.remove(&id);
                        }
                    },
                    Volume(id, vol_val) => {
                        volume(conn, id as u8, vol_val);
                    },
                    Jump(mark) => {
                        silence_all(conn);
                        time = (end_time as f64 * mark).round() as u32;
                        index = starting_index(indexed_timed_messages, time);
                        continue 'outer;
                    },
                    Pan(id, pan_val) => {
                        pan(conn, id as u8, pan_val);
                    },
                }
            }
        }

        // Play
        match message {
            NoteOn{key, vel} if !muted_tracks.contains(track_id) => {
                note_on(conn, *track_id as u8, key.as_int(), vel.as_int());
            },
            NoteOff{key, vel} => {
                note_off(conn, *track_id as u8, key.as_int(), vel.as_int());
            },
            Aftertouch{key, vel} => {
                aftertouch(conn, *track_id as u8, key.as_int(), vel.as_int());
            },
            Controller{controller: contr, value} => {
                controller(conn, *track_id as u8, contr.as_int(), value.as_int());
            },
            ProgramChange{program} => {
                program_change(conn, *track_id as u8, program.as_int());
            },
            ChannelAftertouch{vel} => {
                channel_aftertouch(conn, *track_id as u8, vel.as_int());
            },
            PitchBend{bend} => {
                pitch_bend(conn, *track_id as u8, bend.0.as_int());
            },
            _ => {},
        }

        // Update player state
        time = *new_time;
        index = (index + 1) % indexed_timed_messages.len();
    }
}