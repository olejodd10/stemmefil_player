use midir::MidiOutputConnection;

use midly::{
    TrackEventKind::{
        self, Midi
    }, 
    MidiMessage::{
        NoteOn, NoteOff
    },
};

// crossterm is better than device_query because it only triggers on rising edge
use crossterm::event::{poll, read, Event, KeyCode::Char};

use std::thread::sleep;
use std::time::{Duration, Instant};
use std::collections::HashMap;

use crate::conn::{note_on, note_off, silence};

mod config;
use config::Config;

// Binary search for starting point. Test this.
fn starting_index(indexed_timed_events: &Vec<(usize,u32,TrackEventKind)>, time: u32) -> usize {
    let mut low = 0;
    let mut high = indexed_timed_events.len();
    while high > low + 1 {
        let mid = (low + high)/2;
        if indexed_timed_events[mid].1 >= time {
            high = mid;
        } else {
            low = mid;
        }
    }
    low
}

pub fn play(conn: &mut MidiOutputConnection, indexed_timed_events: &Vec<(usize,u32,TrackEventKind)>, mut time: u32) {
    let start = starting_index(indexed_timed_events, time);
    for (_track_id, new_time, track_event_kind) in &indexed_timed_events[start..] { 
        let sleep_microseconds = new_time.checked_sub(time).unwrap_or(0);
        sleep(Duration::from_micros(sleep_microseconds as u64));
        time = *new_time;
        match track_event_kind {
            Midi{channel: _, message} => {
                match message {
                    NoteOn{key, vel} => note_on(conn, *key, *vel),
                    NoteOff{key, vel} => note_off(conn, *key, *vel),
                    _ => {},
                    // other => println!("Other MidiMessage: {:?}", other), 
                }
            }
            _ => {}, 
        }
    }
}

pub fn player(conn: &mut MidiOutputConnection, indexed_timed_events: &Vec<(usize,u32,TrackEventKind)>) {
    let mut configs: HashMap<usize, Config> = indexed_timed_events.iter().map(|t| (t.0, Config::default())).collect();
    let mut time = 0;
    let mut index = 0;
    let mut paused = false;
    loop { 
        let (_track_id, new_time, track_event_kind) = &indexed_timed_events[index];
        let sleep_duration = Duration::from_micros(new_time.checked_sub(time).unwrap_or(0) as u64);
        let now = Instant::now();
        while now.elapsed() < sleep_duration {
            if let Ok(true) = poll(Duration::from_micros(0)) { // If event is available and read() won't block
                if let Ok(Event::Key(key_event)) = read() {
                    match key_event.code {
                        Char(c) if c == ' ' => {
                            paused = !paused;
                            if paused {
                                silence(conn);
                            }
                        },
                        _ => {},
                    }
                }
            }
        }

        if paused {
            continue;
        }
        time = *new_time;
        match track_event_kind {
            Midi{channel: _, message} => {
                match message {
                    NoteOn{key, vel} => note_on(conn, *key, *vel),
                    NoteOff{key, vel} => note_off(conn, *key, *vel),
                    _ => {},
                    // other => println!("Other MidiMessage: {:?}", other), 
                }
            }
            _ => {}, 
        }
        index = (index + 1) % indexed_timed_events.len();
    }
}