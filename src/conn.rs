use midir::{MidiOutput, MidiOutputPort, MidiOutputConnection};

use midly::num::u7;

use std::io::{stdin, stdout, Write};
use std::error::Error;

// https://www.midi.org/specifications-old/item/table-3-control-change-messages-data-bytes-2
// https://www.midi.org/specifications-old/item/table-2-expanded-messages-list-status-bytes
const NOTE_ON_CHAN_BASE: u8 = 144;
const NOTE_OFF_CHAN_BASE: u8 = 128;
const CONTROL_MODE_CHANGE_CHAN_BASE: u8 = 176;
const PROGRAM_CHANGE_CHAN_BASE: u8 = 192;
const ALL_SOUND_OFF_MSG: u8 = 120;
const PAN_MSG: u8 = 10;
const VOLUME_MSG: u8 = 7;

// Forked from https://github.com/Boddlnagg/midir/blob/master/examples/test_play.rs
pub fn connect() -> Result<MidiOutputConnection, Box<dyn Error>> {
    let midi_out = MidiOutput::new("My Test Output")?;
    
    // Get an output port (read from console if multiple are available)
    let out_ports = midi_out.ports();
    let out_port: &MidiOutputPort = match out_ports.len() {
        0 => return Err("no output port found".into()),
        1 => {
            println!("Choosing the only available output port: {}", midi_out.port_name(&out_ports[0]).unwrap());
            &out_ports[0]
        },
        _ => {
            println!("\nAvailable output ports:");
            for (i, p) in out_ports.iter().enumerate() {
                println!("{}: {}", i, midi_out.port_name(p).unwrap());
            }
            print!("Please select output port: ");
            stdout().flush()?;
            let mut input = String::new();
            stdin().read_line(&mut input)?;
            out_ports.get(input.trim().parse::<usize>()?)
                     .ok_or("invalid output port selected")?
        }
    };
    
    println!("Opening connection");
    Ok(midi_out.connect(out_port, "midir-test")?)
}

pub fn note_on(conn: &mut MidiOutputConnection, chan: u8, key: u7, vel: u7) {
    let _ = conn.send(&[NOTE_ON_CHAN_BASE+chan, key.as_int(), vel.as_int()]);
}

pub fn note_off(conn: &mut MidiOutputConnection, chan: u8, key: u7, vel: u7) {
    let _ = conn.send(&[NOTE_OFF_CHAN_BASE+chan, key.as_int(), vel.as_int()]);
}

pub fn silence(conn: &mut MidiOutputConnection, chan: u8) {
    let _ = conn.send(&[CONTROL_MODE_CHANGE_CHAN_BASE + chan, ALL_SOUND_OFF_MSG, 0]);
}

pub fn silence_all(conn: &mut MidiOutputConnection) {
    // Omni mode and then silence doesn't work...
    for chan in 0..16 {
        let _ = silence(conn, chan);
    }
}

pub fn pan(conn: &mut MidiOutputConnection, chan: u8, val: u8) {
    let _ = conn.send(&[CONTROL_MODE_CHANGE_CHAN_BASE + chan, PAN_MSG, val]);
}

pub fn volume(conn: &mut MidiOutputConnection, chan: u8, val: u8) {
    let _ = conn.send(&[CONTROL_MODE_CHANGE_CHAN_BASE + chan, VOLUME_MSG, val]);
}

pub fn program_change(conn: &mut MidiOutputConnection, chan: u8, val: u8) {
    let _ = conn.send(&[PROGRAM_CHANGE_CHAN_BASE + chan, val]);
}

pub fn controller(conn: &mut MidiOutputConnection, chan: u8, controller: u8, val: u8) {
    let _ = conn.send(&[CONTROL_MODE_CHANGE_CHAN_BASE + chan, controller, val]);
}