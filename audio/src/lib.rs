pub mod audio_manager;
pub mod sound_store;
pub mod cached_sound;
pub mod end_play_sound;
pub mod commands;

pub use audio_manager::AudioManager;
pub use sound_store::SoundStore;
pub use cached_sound::CachedSound;
pub use end_play_sound::EndPlaySound;


const ASSETS_DIR: &str = "assets";


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
