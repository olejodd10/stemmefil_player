mod conn;
mod parse;
mod play;
mod gui;
mod command;

use std::fs::read;
use std::env::args;
use std::path::Path;
use std::sync::mpsc::sync_channel;

const COMMAND_CHANNEL_BOUND: usize = 8;

fn main() {
    let mut conn = conn::connect().unwrap();
    let midi_path = args().nth(1).expect("Error: No MIDI path provided");
    let bytes = read(Path::new(&midi_path)).expect("Error reading MIDI file");
    let messages = parse::parse_to_indexed_timed_messages(&bytes);
    let trck_names = parse::track_names(&bytes);
    let song_name = trck_names.get(&0).unwrap().to_string();
    let (send, recv) = sync_channel(COMMAND_CHANNEL_BOUND);
    std::thread::spawn(move || {
        play::play_real_time(&mut conn, &messages, recv);
    });
    gui::run(song_name, trck_names, send);
}
