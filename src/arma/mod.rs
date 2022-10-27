// pub mod bot;
pub mod config;
pub mod commands;

pub mod prelude {
    // pub use crate::arma::bot::Manager;
    // pub use crate::arma::bot::ArmaServerDescriptor;
}

use serenity::model::id::GuildId;
use std::env;

#[derive(Clone)]
pub struct ArmaDiscordConfiguration {
    pub steam_api_key: String,
    pub arma_server_host: String,
    pub discord_guild_id: GuildId,
}

impl ArmaDiscordConfiguration {
    // TODO: Use SQL and inject the info at start-up somehow, creating a vector of this info from
    // TODO: the state, ready and guild data
    pub fn new() -> Option<ArmaDiscordConfiguration> {
        // No point if we can't even hit the API
        if let Ok(steam_api_key) = env::var("STEAM_API_KEY") {
            // Or don't know what to request
            if let Ok(arma_server_host) = env::var("ARMA_HOST_STRING") {
                // And haven't figured out a guild that manages this server ...
                if let Ok(guild_id_raw) = env::var("GUILD_ID").ok()?.parse() {
                    return Some(ArmaDiscordConfiguration{
                        steam_api_key,
                        arma_server_host,
                        discord_guild_id: GuildId(guild_id_raw),
                    })

                }
            }
        }
        None
    }
}