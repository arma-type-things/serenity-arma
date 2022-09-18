use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use crate::arma::ArmaDiscordConfiguration;

// pub struct PingCommand;
//
// impl PingCommand {
//     pub fn run(_options: &[CommandDataOption]) -> String {
//         "Pong!".to_string()
//     }
//     pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
//         command.name("ping").description("Ping the bot to test uptime and lag.")
//     }
// }

pub struct ServerStatusCommand;

impl ServerStatusCommand {

    pub fn run(_options: &[CommandDataOption], arma_configuration: &ArmaDiscordConfiguration) -> String {
        format!("not yet implemented, but I am configured to collect server information from {}", arma_configuration.arma_server_host)
    }
    pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
        command
            .name("status")
            .description("Get status of a server or the default server.")
    }
}