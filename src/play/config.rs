pub struct Config {
    gain: f64,
    muted: bool,
    // pitch: ,
    // speed: ,
    // solo: bool, // TODO
}

impl Default for Config {
    fn default() -> Self {
        Config {
            gain: 1.0,
            muted: false,
        }
    }
}