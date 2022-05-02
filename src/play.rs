use midir::MidiOutputConnection;

use midly::{
    Header,
    TrackEventKind::{
        self, Midi, Meta
    }, 
    MidiMessage::{
        NoteOn, NoteOff
    },
    MetaMessage::Tempo
};

use std::thread::sleep;
use std::time::Duration;

mod safe_timing;
use safe_timing::SafeTiming;
use crate::conn::{note_on, note_off};


pub fn play(conn: &mut MidiOutputConnection, header: Header, indexed_timed_events: Vec<(usize,u32,TrackEventKind)>, mut ticks: u32) {
    let mut timing = SafeTiming::from_header(&header); 
    for (track_id, new_ticks, track_event_kind) in indexed_timed_events.into_iter().skip_while(move |(_, new_ticks, _)| *new_ticks < ticks) { // Check that this doesn't eat one too many
        sleep(Duration::from_micros(((new_ticks - ticks)*timing.microsecs_per_tick()) as u64)); // Relate this to tempo        
        ticks = new_ticks;
        match track_event_kind {
            Midi{channel: _, message} => {
                match message {
                    NoteOn{key, vel} => note_on(conn, key, vel),
                    NoteOff{key, vel} => note_off(conn, key, vel),
                    _ => {},
                    // other => println!("Other MidiMessage: {:?}", other), 
                }
            }
            Meta(meta_message) => {
                match meta_message {
                    Tempo(microsecs_per_beat) => {
                        timing.set_microsecs_per_beat(microsecs_per_beat);
                    },
                    // These are actually only metadata. 
                    // TimeSignature(_num, _den, _clocks_per_click, _thirtyseconds_per_quarter) => {
                    //     // unimplemented!("");
                    // },
                    // KeySignature(_sharps, _major) => {
                    //     // unimplemented!("");
                    // },
                    _ => {},
                    // other => println!("Other MetaMessage: {:?}", other), 
                }
            }
            _ => {}, // Implement
        }
    }
}