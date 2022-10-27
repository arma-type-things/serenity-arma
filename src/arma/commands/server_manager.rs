use std::fmt::{Debug, Display, Formatter};
use serenity::builder::CreateApplicationCommand;
use serenity::model::application::interaction::Interaction;
use serenity::model::prelude::GuildId;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::interaction::InteractionResponseType;
use serenity::prelude::Context;
use sqlx::SqlitePool;
use crate::ArmaServerDescriptor;
use crate::commands::steam::{GetServersAtAddressResponse, SteamResponse};

pub struct ServerCommandError {
    pub message: String
}

impl Debug for ServerCommandError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl Display for ServerCommandError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for ServerCommandError {}

pub(crate) struct ServerManagerCommand {
    database_pool: &'static SqlitePool,
}

impl ServerManagerCommand {
    pub fn new(database_pool: &'static SqlitePool) -> Self {
        Self {
            database_pool
        }
    }

    pub async fn handle(&self, ctx: &Context, command: &ApplicationCommandInteraction, guild_configuration: Option<ArmaServerDescriptor>) -> Result<(), ServerCommandError> {
        let sub_command = command.data.options.get(0).unwrap();
        match sub_command.name.as_str() {
            "query" => { self.handle_server_query(command, guild_configuration).await },
            _ => { Err(ServerCommandError { message: "not implemented".to_string() }) }
        }
    }

    pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
        command
            .name("server")
            .description("Server management commands.")
    }


    async fn handle_server_query(&self, command: &ApplicationCommandInteraction, guild_configuration: Option<ArmaServerDescriptor>) -> Result<(), ServerCommandError> {
        let guild_configuration = guild_configuration.ok_or(ServerCommandError { message: "no configuration found for this guild".to_string() })?;
        let steam_response =
            Self::fetch_servers_at_address(&guild_configuration).await;
        match steam_response {
            Ok(response) =>  {
                Ok(())
            }
            Err(why) => {
                Err(ServerCommandError {
                    message: format!("Steam error fetching server query: {}", why.to_string())
                })
            }
        }

    }

    async fn fetch_servers_at_address(arma_configuration: &ArmaServerDescriptor) -> Result<SteamResponse<GetServersAtAddressResponse>, reqwest::Error> {
        reqwest::get(
            format!("https://api.steampowered.com/ISteamApps/GetServersAtAddress/v0001?addr={}", arma_configuration.arma_server_host))
            .await?
            .json::<SteamResponse<GetServersAtAddressResponse>>().await
    }
}