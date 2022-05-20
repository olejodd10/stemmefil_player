pub enum Command {
    Play,
    Pause,
    Muted(usize, bool),
    Volume(usize, u8),
    Jump(f64),
    Pan(usize, u8),
}