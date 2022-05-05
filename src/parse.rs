use midly::{
    Format, Smf, Track, TrackEvent, TrackEventKind, TrackEventKind::Meta, MetaMessage::Tempo, 
};

mod safe_timing;
use safe_timing::SafeTiming;


fn ticked_events(track: Track) -> Vec<(u32, TrackEventKind)> {
    let mut ticks = 0;
    track.into_iter().map(|TrackEvent{delta, kind}| {
        ticks += delta.as_int();
        (ticks, kind)
    }).collect()
}

pub fn parse_to_indexed_timed_events(bytes: &[u8]) -> Vec<(usize,u32,TrackEventKind)> {
    let Smf{header, tracks} = midly::Smf::parse(bytes).expect("Error parsing MIDI bytes");
    if header.format != Format::Parallel { // TODO
        panic!("Error: Expected parallel track");
    }

    let mut indexed_ticked_events = Vec::new();
    for (id, track) in tracks.into_iter().enumerate() {
        indexed_ticked_events.extend(ticked_events(track).into_iter().map(|(t1,t2)| (id,t1,t2)))
    }
    // Optionally remove unecessary events
    indexed_ticked_events.sort_by(|t1,t2| t1.1.cmp(&t2.1));
    // for (i, v) in indexed_ticked_events.iter().enumerate() {
    //     println!("{}: {}", i, v.1);
    // }

    // Time depends on microsecs_per_beat, which again might depend on Tempo events on other tracks. Therefore this function needs to consider all tracks at the same time
    let mut timing = SafeTiming::from_header(&header); 
    let mut ticks = 0;
    let mut time = 0;
    indexed_ticked_events.into_iter().map(|(id, new_ticks, kind)| {
        let delta = new_ticks.checked_sub(ticks).unwrap_or(0);
        time += delta*timing.microsecs_per_tick(); // Delta answers "how many ticks after the previous event it should this fire?"
        ticks = new_ticks;
        if let Meta(Tempo(microsecs_per_beat)) = kind { 
            // println!("Timing: {:?}, microsecs per tick {}", timing, timing.microsecs_per_tick());
            timing.set_microsecs_per_beat(microsecs_per_beat);
        }
        (id, time, kind)
    }).collect()
}