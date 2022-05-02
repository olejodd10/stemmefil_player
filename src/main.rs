mod conn;
mod parse;
mod play;

use std::fs::read;
use std::path::Path;

fn main() {
    let mut conn = conn::connect().unwrap();
    let bytes = read(Path::new("busen.mid")).expect("Error reading MIDI file");
    let (header, events) = parse::parse_into_events(&bytes);
    play::play(&mut conn, header, events, 0);
}
