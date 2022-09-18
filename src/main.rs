mod commands;
mod handler;
mod arma;

use std::env;
use serenity::Client;
use serenity::prelude::GatewayIntents;
use handler::Handler;

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("A Discord token must be set");
    let intents = GatewayIntents::GUILDS
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler::new())
        .await
        .expect("Error creating Discord client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

