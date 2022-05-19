pub enum Command {
    Play,
    Pause,
    Muted(usize, bool),
    Gain(usize, f64),
    Jump(f64),
}