pub struct Config {
    pub volume: u8,
    pub muted: bool,
    pub pan: u8,
    // pitch: ,
    // speed: ,
    // solo: Option<bool>, // TODO. Radio buttons.
}

impl Default for Config {
    fn default() -> Self {
        Config {
            volume: 63,
            muted: false,
            pan: 63,
        }
    }
}