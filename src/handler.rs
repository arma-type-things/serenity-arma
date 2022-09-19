use std::string::ToString;
use serenity::{
    async_trait,
    model::{
        application::{
            // command::Command,
            interaction::{
                Interaction,
                InteractionResponseType
            }
        },
        gateway::Ready,
        id::GuildId
    },
    prelude::*
};

use crate::arma::ArmaDiscordConfiguration;
use crate::commands::steam::{ServerQueryCommand, ServerStatusCommand};

pub(crate) struct Handler {
    pub(crate) arma_discord_configuration: Option<ArmaDiscordConfiguration>
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready_data: Ready) {
        if let Some(shard) = ready_data.shard {
            println!("{} is connected on shard {}/{}.", ready_data.user.name, shard[0], shard[1]);
            self.inject_arma_commands(&ctx).await;
            self.inject_global_commands(&ctx).await;
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let content = match command.data.name.as_str() {
                "query" => {
                    match &self.arma_discord_configuration {
                        Some(config) => {
                            ServerQueryCommand::run(&command.data.options, config).await
                        },
                        _ => "no valid configuration found for query command".to_string()
                    }
                },
                "status" => {
                    match &self.arma_discord_configuration {
                        Some(config) => {
                            ServerStatusCommand::run(&command.data.options, config).await
                        },
                        _ => "no valid configuration found for status command".to_string()
                    }
                }
                _ => "not implemented".to_string(),
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response | {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await {
                println!("Cannot respond to slash command: {}", why)
            };
        }
    }
}

impl Handler {
    pub fn new() -> Self {
        Handler {
            arma_discord_configuration: ArmaDiscordConfiguration::new()
        }
    }

    async fn inject_arma_commands(&self, ctx: &Context) {
        match &self.arma_discord_configuration {
            Some(config) => {
                print!("Injecting ArmA commands for Guild {}: ", config.discord_guild_id);
                let _ = GuildId::set_application_commands(
                    &config.discord_guild_id,
                    &ctx.http,
                    |commands| {
                        print!(".. /query ");
                        commands
                            .create_application_command(|command| ServerQueryCommand::register(command));
                        print!(".. /status ");
                        commands
                            .create_application_command(|command| ServerStatusCommand::register(command))
                }).await;
                println!("... done.");
            },
            _ => {}
        }
    }

    async fn inject_global_commands(&self, _ctx: &Context) {
        print!("Injecting global commands: ");
        // TODO: Define global commands, if any.
        println!(".. none to inject.");
    }
}