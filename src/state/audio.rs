use crate::message::audio::AudioTo;
#[derive(Clone, Debug)]
pub enum Audio {
    Playing,
    Pausing, 
    Stopping,
}
impl From<AudioTo> for Audio {
    fn from(audio_to: AudioTo) -> Self {
        match audio_to {
            AudioTo::Play => Audio::Playing,
            AudioTo::Stop => Audio::Stopping,
            AudioTo::Pause => Audio::Pausing,
        }
    }
}
