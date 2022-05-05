mod conn;
mod parse;
mod play;

use std::fs::read;
use std::path::Path;

fn main() {
    let mut conn = conn::connect().unwrap();
    let bytes = read(Path::new("kunglil.mid")).expect("Error reading MIDI file");
    let events = parse::parse_to_indexed_timed_events(&bytes);
    play::play(&mut conn, &events, 35000000);
}
