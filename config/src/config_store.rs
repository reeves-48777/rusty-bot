use serenity::prelude::TypeMapKey;
use std::sync::Arc;
use tokio::sync::RwLock;
use super::Configuration;
pub struct ConfigStore;

impl TypeMapKey for ConfigStore {
    type Value = Arc<RwLock<Configuration>>;
}