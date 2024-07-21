pub enum Settings {
    Own,
    Audio, //(Audio)
    Common,
    Advanced,
    About,
}
impl Default for Settings {
    fn default() -> Self {
        Self::Own
    }
}
// pub enum Audio {}
