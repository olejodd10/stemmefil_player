use midly::{Timing, Header, num::u24};

#[derive(Debug)]
pub enum SafeTiming {
    Metrical(u32, u32), // (microsecs per beat, ticks per beat) 
    Explicit(u32)
}

impl SafeTiming {
    pub fn from_header(header: &Header) -> Self {
        match header.timing {
            Timing::Metrical(ticks_per_beat) => { // "The length of a beat is not standard, so in order to fully describe the length of a MIDI tick the MetaMessage::Tempo event should be present."
                SafeTiming::Metrical(1000_000, ticks_per_beat.as_int() as u32)
            },
            Timing::Timecode(fps, subframe) => SafeTiming::Explicit(1000_000/(fps.as_int() as u32)/(subframe as u32)),
        }
    }

    pub fn microsecs_per_tick(&self) -> u32 {
        match self {
            SafeTiming::Metrical(microsecs_per_beat, ticks_per_beat) => microsecs_per_beat/ticks_per_beat,
            SafeTiming::Explicit(microsecs_per_tick) => *microsecs_per_tick,
        }
    }

    pub fn set_microsecs_per_beat(&mut self, new_microsecs_per_beat: u24) {
        match self {
            SafeTiming::Metrical(microsecs_per_beat, _) => {
                *microsecs_per_beat = new_microsecs_per_beat.as_int();
            },
            SafeTiming::Explicit(_) => println!("WARNING: Tempo message, but ticks per beat is unknown")
        }
    }
}