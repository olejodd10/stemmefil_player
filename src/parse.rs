use midly::{
    Format, Smf, Track, TrackEvent, TrackEventKind, Header
};


fn timed_events(track: Track) -> Vec<(u32, TrackEventKind)> {
    let mut ticks = 0;
    track.into_iter().map(|TrackEvent{delta, kind}| {
        ticks += delta.as_int();
        (ticks, kind)
    }).collect()
}

pub fn parse_into_events(bytes: &[u8]) -> (Header, Vec<(usize,u32,TrackEventKind)>) {
    let Smf{header, tracks} = midly::Smf::parse(bytes).expect("Error parsing MIDI bytes");
    if header.format != Format::Parallel { // TODO
        panic!("Error: Expected parallel track");
    }

    let mut indexed_timed_events = Vec::new();
    for (id, track) in tracks.into_iter().enumerate() {
        indexed_timed_events.extend(timed_events(track).into_iter().map(|(t1,t2)| (id,t1,t2)))
    }
    // Optionally remove unecessary events
    indexed_timed_events.sort_by(|t1,t2| t1.1.cmp(&t2.1));
    (header, indexed_timed_events)
}