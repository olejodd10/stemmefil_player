use midir::MidiOutputConnection;

use midly::{
    TrackEventKind::{
        self, Midi
    }, 
    MidiMessage::{
        NoteOn, NoteOff
    },
};

use std::thread::sleep;
use std::time::Duration;

use crate::conn::{note_on, note_off};

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

// Be careful with the starting ticks. It will fast forward until it hits a tempo message.
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