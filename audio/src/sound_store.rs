use serenity::prelude::TypeMapKey;
use std::sync::Arc;
use tokio::sync::RwLock;

use std::collections::HashMap;
use super::CachedSound;

pub struct SoundStore;

impl TypeMapKey for SoundStore {
	type Value = Arc<RwLock<HashMap<String, CachedSound>>>;
}