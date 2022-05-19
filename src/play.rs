use midir::MidiOutputConnection;

use midly::{
    MidiMessage::{
        self, NoteOn, NoteOff
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

pub fn play(conn: &mut MidiOutputConnection, indexed_timed_messages: &[(usize,u32,MidiMessage)], mut time: u32) {
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

pub fn player(conn: &mut MidiOutputConnection, indexed_timed_messages: &[(usize,u32,MidiMessage)]) {
    let mut configs: HashMap<usize, Config> = indexed_timed_messages.iter().map(|t| (t.0, Config::default())).collect();
    let mut time = 0;
    let mut index = 0;
    let mut paused = false;
    loop { 
        let (_track_id, new_time, message) = &indexed_timed_messages[index];
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
        match message {
            NoteOn{key, vel} => note_on(conn, *key, *vel),
            NoteOff{key, vel} => note_off(conn, *key, *vel),
            _ => {},
            // other => println!("Other MidiMessage: {:?}", other), 
        }
        index = (index + 1) % indexed_timed_messages.len();
    }
}