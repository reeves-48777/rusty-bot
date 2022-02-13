#[cfg(feature = "audio")]
use audio::AudioManager;

use config::{ConfigBuilder, Configuration};
use super::handler::Handler;

use serenity::framework::StandardFramework;
use serenity::http::Http;
use serenity::cache::Cache;
use serenity::async_trait;
use serenity::Client;

use std::env;
use std::collections::HashSet;


#[async_trait]
pub trait Bot {
    async fn run(&self) {}
}

pub struct CommonBot {
    pub token: String,
    pub config: Configuration,
    pub http: Http,
    pub cache: Cache,
}

impl CommonBot {
    pub fn new() -> Self {
        let token = env::var("DISCORD_BOT_TOKEN").expect("DISCORD_BOT_TOKEN env var set");
        Self {
            token: token.clone(),
            config: ConfigBuilder::new().build(),
            http: Http::new_with_token(&token),
            cache: Cache::new()
        }
    }
}

#[cfg(feature = "audio")]
pub struct AudioBot {
    pub token: String,
    pub audio_manager: AudioManager,
    pub config: Configuration,
    pub http: Http,
    pub cache: Cache
}

#[cfg(feature = "audio")]
impl AudioBot {
    fn new() -> Self {
        let token = env::var("DISCORD_BOT_TOKEN").expect("DISCORD_BOT_TOKEN env var set");
        Self {
            token: token.clone(),
            audio_manager: AudioManager::new(),
            config: ConfigBuilder::new().build(),
            http: Http::new_with_token(&token),
            cache: Cache::new()
        }
    }
}

#[async_trait]
impl Bot for CommonBot {
    async fn run(&self) {
        let (owners, _bot_id) = match self.http.get_current_application_info().await {
            Ok(info) => {
                let mut owners = HashSet::new();
                if let Some(team) = info.team {
                    owners.insert(team.owner_user_id);
                } else {
                    owners.insert(info.owner.id);
                }
                match self.http.get_current_user().await {
                    Ok(bot_id) => (owners, bot_id),
                    Err(why) => panic!("Could not access the bot: {:?}", why)
                }
            },
            Err(why) => panic!("Could not get app ingo: {:?}", why)
        };

        use common::hooks::unknown_command;
        use common::commands::{MY_HELP, GENERAL_GROUP};
        use config::commands::CONFIG_GROUP;
        let framework = StandardFramework::new()
            .configure(|c| c
                .prefix("$")
                .owners(owners))
            .unrecognised_command(unknown_command)
            .help(&MY_HELP)
            .group(&GENERAL_GROUP)
            .group(&CONFIG_GROUP);
        
        let mut client = {
            Client::builder(self.token.clone())
            .event_handler(Handler)
            .framework(framework)
            .await
            .expect("Error creating client")
        };

        {
            use config::ConfigStore;
            use std::sync::Arc;
            use tokio::sync::RwLock;
            let mut data = client.data.write().await;
		
            // NOTE audio map should be in the bot struct
            // and should be initialized empty
            let bot_config = ConfigBuilder::new().build();
            
            data.insert::<ConfigStore>(Arc::new(RwLock::new(bot_config)));
        }

        tokio::spawn(async move { 
            let _ = client.start().await.map_err(|why| println!("Client ended: {:?}", why));
        });

        tokio::signal::ctrl_c().await.unwrap();
        println!("Ctrl-C received, shutting down");
    }
}

#[cfg(feature = "audio")]
#[async_trait]
impl Bot for AudioBot {
    async fn run(&self) {
        let (owners, _bot_id) = match self.http.get_current_application_info().await {
            Ok(info) => {
                let mut owners = HashSet::new();
                if let Some(team) = info.team {
                    owners.insert(team.owner_user_id);
                } else {
                    owners.insert(info.owner.id);
                }
                match self.http.get_current_user().await {
                    Ok(bot_id) => (owners, bot_id),
                    Err(why) => panic!("Could not access the bot: {:?}", why)
                }
            },
            Err(why) => panic!("Could not get app ingo: {:?}", why)
        };

        use common::hooks::unknown_command;
        use common::commands::{MY_HELP, GENERAL_GROUP};
        use config::commands::CONFIG_GROUP;
        use songbird::serenity::SerenityInit;
        let framework = StandardFramework::new()
            .configure(|c| c
                .prefix("$")
                .owners(owners))
            .unrecognised_command(unknown_command)
            .help(&MY_HELP)
            .group(&GENERAL_GROUP)
            .group(&CONFIG_GROUP);

        let mut client = {
            Client::builder(self.token.clone())
                .event_handler(Handler)
                .framework(framework)
                .register_songbird()
                .await
                .expect("Error creating client")
        };

        {
            use config::ConfigStore;
            use std::sync::Arc;
            use tokio::sync::RwLock;
            let mut data = client.data.write().await;

            // NOTE audio map should be in the bot struct
            // and should be initialized empty
            let bot_config = ConfigBuilder::new().build();
            let audio_map = todo!();

            data.insert::<ConfigStore>(Arc::new(RwLock::new(bot_config)));
            data.insert::<SoundStore>(Arc::new(Mutex(audio_map)));
        }

        tokio::spawn(async move {
            let _ = client.start().await.map_err(|why| println!("Client ended: {:?}", why));
        });

        tokio::signal::ctrl_c().await.unwrap();
        println!("Ctrl-C received, shutting down");
    }
}