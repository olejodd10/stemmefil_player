use midir::{MidiOutput, MidiOutputPort, MidiOutputConnection};

use midly::num::u7;

use std::io::{stdin, stdout, Write};
use std::error::Error;

const NOTE_ON_MSG: u8 = 0x90;
const NOTE_OFF_MSG: u8 = 0x80;

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

pub fn note_on(conn: &mut MidiOutputConnection, key: u7, vel: u7) {
    let _ = conn.send(&[NOTE_ON_MSG, key.as_int(), vel.as_int()]);
}

pub fn note_off(conn: &mut MidiOutputConnection, key: u7, vel: u7) {
    let _ = conn.send(&[NOTE_OFF_MSG, key.as_int(), vel.as_int()]);
}
