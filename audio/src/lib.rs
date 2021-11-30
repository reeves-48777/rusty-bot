


#[cfg(feature="audio")]
pub mod audio_manager;
#[cfg(feature="audio")]
pub mod sound_store;
#[cfg(feature="audio")]
pub mod cached_sound;
#[cfg(feature="audio")]
pub mod end_play_sound;
#[cfg(feature="audio")]
pub mod commands;

#[cfg(feature="audio")]
pub use audio_manager::AudioManager;
#[cfg(feature="audio")]
pub use sound_store::SoundStore;
#[cfg(feature="audio")]
pub use cached_sound::CachedSound;
#[cfg(feature="audio")]
pub use end_play_sound::EndPlaySound;

#[cfg(feature="audio")]
const ASSETS_DIR: &str = "assets";


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
