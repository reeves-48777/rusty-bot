pub mod configuration;
pub mod config_builder;
pub mod config_store;
pub mod commands;

pub use configuration::Configuration;
pub use config_builder::ConfigBuilder;
pub use config_store::ConfigStore;




#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
