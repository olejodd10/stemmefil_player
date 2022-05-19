use midir::MidiOutputConnection;

use midly::{
    MidiMessage::{
        self, NoteOn, NoteOff
    },
    num::u7,
};

use std::thread::sleep;
use std::time::{Duration, Instant};
use std::collections::HashMap;
use std::sync::mpsc::Receiver;

use crate::conn::{note_on, note_off, silence};

use crate::config::Config;
use crate::command::Command::{
    self,
    Play,
    Pause,
    Muted,
    Gain,
    Jump,
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

pub fn play_simple(conn: &mut MidiOutputConnection, indexed_timed_messages: &[(usize,u32,MidiMessage)], mut time: u32) {
    let start = starting_index(indexed_timed_messages, time);
    for (_track_id, new_time, message) in &indexed_timed_messages[start..] { 
        let sleep_microseconds = new_time.checked_sub(time).unwrap_or(0);
        sleep(Duration::from_micros(sleep_microseconds as u64));
        time = *new_time;
        match message {
            NoteOn{key, vel} => note_on(conn, *key, *vel),
            NoteOff{key, vel} => note_off(conn, *key, *vel),
            _ => {},
            // other => println!("Other MidiMessage: {:?}", other), 
        }
    }
}

pub fn play_real_time(conn: &mut MidiOutputConnection, indexed_timed_messages: &[(usize,u32,MidiMessage)], recv: Receiver<Command>) {
    let mut configs: HashMap<usize, Config> = indexed_timed_messages.iter().map(|t| (t.0, Config::default())).collect();
    let end_time = indexed_timed_messages.last().unwrap().1;
    let mut time = 0;
    let mut index = 0;
    let mut paused = false;
    'outer: loop { 
        let (track_id, new_time, message) = &indexed_timed_messages[index];
        let sleep_duration = Duration::from_micros(new_time.checked_sub(time).unwrap_or(0) as u64);
        let now = Instant::now();
        while now.elapsed() < sleep_duration {
            // Listen for commands
            if let Ok(command) = recv.try_recv() {
                match command {
                    Play => paused = false,
                    Pause => {
                        silence(conn);
                        paused = true;
                    },
                    Muted(id, muted) => configs.get_mut(&id).unwrap().muted = muted,
                    Gain(id, gain) => configs.get_mut(&id).unwrap().gain = gain,
                    Jump(mark) => {
                        silence(conn);
                        time = (end_time as f64 * mark).round() as u32;
                        index = starting_index(indexed_timed_messages, time);
                        continue 'outer;
                    },
                }
            }
        }

        // Play
        if paused {
            continue;
        }
        time = *new_time;
        let config = configs.get(track_id).unwrap();
        match message {
            NoteOn{key, vel} if !config.muted => {
                let modified_vel = u7::from_int_lossy((vel.as_int() as f64 * config.gain).round() as u8);
                note_on(conn, *key, modified_vel)
            },
            NoteOff{key, vel} => note_off(conn, *key, *vel),
            _ => {},
            // other => println!("Other MidiMessage: {:?}", other), 
        }
        index = (index + 1) % indexed_timed_messages.len();
    }
}