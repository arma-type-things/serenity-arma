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
use serenity::model::prelude::CurrentUser;

// Config defines the basic configuration for the bot.
#[derive(Clone)]
pub struct Config {
    pub token: String,
    pub prefix: String,
    pub steam_api_key: String,
    pub arma_server_host: String,
    pub guild_id: GuildId,
}

// ArmaServerDescriptor defines the basic information about the Arma server.
#[derive(Clone)]
pub struct ArmaServerDescriptor {
    pub host: String,
    pub port: u16,
    pub password: String,
    pub rcons_password: String,
}

pub struct Bot {
    pub config: Config,
    pub arma_server: ArmaServerDescriptor,
}

impl Bot {
    pub fn new(config: Config,  arma_server: ArmaServerDescriptor) -> Self {
        Bot {
            config,
            arma_server,
        }
    }

    pub fn get_guild_id(&self) -> GuildId {
        self.config.guild_id
    }
}

#[async_trait]
impl EventHandler for Bot {
    async fn ready(&self, ctx: Context, ready_data: Ready) {
        if let Some(shard) = ready_data.shard {
            println!("{} is connected on shard {}/{}.", ready_data.user.name, shard[0], shard[1]);
            // self.inject_arma_commands(&ctx).await;
            // self.inject_global_commands(&ctx).await;
        }
    }

    async fn user_update(&self, _ctx: Context, _new_data: CurrentUser) {
        todo!()
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            // let content = match command.data.name.as_str() {
            //     "query" => {
            //         match &self.arma_discord_configuration {
            //             Some(config) => {
            //                 ServerQueryCommand::run(&command.data.options, config).await
            //             },
            //             _ => "no valid configuration found for query command".to_string()
            //         }
            //     },
            //     "status" => {
            //         match &self.arma_discord_configuration {
            //             Some(config) => {
            //                 ServerStatusCommand::run(&command.data.options, config).await
            //             },
            //             _ => "no valid configuration found for status command".to_string()
            //         }
            //     }
            //     _ => "not implemented".to_string(),
            // };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response | {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content("test"))
                })
                .await
            {
                println!("Cannot respond to command: {:?}", why);
            }
        }
    }
}

