mod conn;
mod parse;
mod play;

use std::fs::read;
use std::path::Path;

fn main() {
    let mut conn = conn::connect().unwrap();
    let bytes = read(Path::new("busen.mid")).expect("Error reading MIDI file");
    let events = parse::parse_to_indexed_timed_events(&bytes);
    play::player(&mut conn, &events);
}
