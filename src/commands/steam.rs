use serde::Deserialize;
use serenity::model::application::interaction::application_command::CommandDataOption;
use serenity::utils::MessageBuilder;
use serenity::builder::CreateApplicationCommand;
use crate::arma::ArmaDiscordConfiguration;

pub struct ServerQueryCommand;

impl ServerQueryCommand {
    pub async fn run(_options: &[CommandDataOption], arma_configuration: &ArmaDiscordConfiguration) -> String {
        match Self::fetch_steam_data(arma_configuration).await {
            Ok(steam_response) => MessageBuilder::new()
                .push_bold_line("Steam Response:")
                .push_codeblock_safe(format!("{:#?}", steam_response), Some("json"))
                .build(),
            Err(why) => why.to_string(),
        }
    }
    pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
        command
            .name("query")
            .description("Query the server to list all running instances.")
    }
    async fn fetch_steam_data(arma_configuration: &ArmaDiscordConfiguration) -> Result<SteamResponse<GetServersAtAddressResponse>, reqwest::Error> {
        reqwest::get(
            format!("https://api.steampowered.com/ISteamApps/GetServersAtAddress/v0001?addr={}", arma_configuration.arma_server_host))
            .await?
            .json::<SteamResponse<GetServersAtAddressResponse>>().await
    }
}

pub struct ServerStatusCommand;

impl ServerStatusCommand {
    pub async fn run(_options: &[CommandDataOption], arma_configuration: &ArmaDiscordConfiguration) -> String {
        match Self::fetch_steam_data(arma_configuration).await {
            Ok(steam_response) => {
                if let Some(servers) = steam_response.response.servers {
                    if let Some(server) = servers.first() {
                        return MessageBuilder::new()
                            .push_line("Sure!")
                            .push_bold_line(format!("Server Status for {}:", server.name))
                            .push_bold("Map: ")
                            .push_line(format!("{}", server.map))
                            .push_bold("Players: ")
                            .push_line(format!("{}/{}", server.players, server.max_players))
                            .push_bold("Connect: ")
                            .push_line(format!("steam://connect/{}", arma_configuration.arma_server_host))
                            .build()
                    }
                }
                "no server found or the server is down, sorry!".to_string()
            },
            Err(why) => why.to_string(),
        }
    }
    pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
        command
            .name("status")
            .description("Query the server to list all running instances.")
    }
    async fn fetch_steam_data(arma_configuration: &ArmaDiscordConfiguration) -> Result<SteamResponse<GetServerListResponse>, reqwest::Error> {
        reqwest::get(
            format!(
                "https://api.steampowered.com/IGameServersService/GetServerList/v1?key={}&filter=addr\\{}",
                arma_configuration.steam_api_key,
                arma_configuration.arma_server_host))
            .await?
            .json::<SteamResponse<GetServerListResponse>>().await
    }

}

// Steam structures
/// SteamResponse is a wrapper around the actual response, representing what Steam Web API returns.
#[derive(Deserialize, Debug)]
pub struct SteamResponse<T: SteamApiResponse> {
    pub response: T
}

/// SteamApiResponse is an empty trait representing the response data from the Steam Web API.
pub trait SteamApiResponse {}

/// GetServerListResponse is the actual response data generated when calling the Steam Web API's GetServerList endpoint.
/// It contains a list of servers.
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct GetServerListResponse {
    servers: Option<Vec<SteamServer>>
}

impl SteamApiResponse for GetServerListResponse {}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct SteamServer {
    pub addr: String,
    pub gameport: u32,
    pub steamid: String,
    pub name: String,
    pub appid: u32,
    pub gamedir: String,
    pub version: String,
    pub product: String,
    pub region: i32,
    pub players: u32,
    pub max_players: u32,
    pub bots: u32,
    pub map: String,
    pub secure: bool,
    pub dedicated: bool,
    pub os: String,
    pub gametype: String,
}

/// GetServersAtAddressResponse is the actual response data generated when calling the Steam Web API's GetServersAtAddress endpoint.
/// it has two fields, success, which is a boolean indicating whether the request was successful, and servers, which is a vector of servers.
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub(crate) struct GetServersAtAddressResponse {
    pub(crate) success: bool,
    pub(crate) servers: Option<Vec<ServersAtAddress>>
}

impl SteamApiResponse for GetServersAtAddressResponse {}

/// ServersAtAddress represents a single server as returned by the Steam Web API's GetServersAtAddress endpoint.
/// It has multiple fields, including the server's IP address, port, and game ID.
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub(crate) struct ServersAtAddress {
    pub(crate) addr: String,
    pub(crate) gmsindex: i32,
    pub(crate) steamid: String,
    pub(crate) appid: i32,
    pub(crate) gamedir: String,
    pub(crate) region: i32,
    pub(crate) secure: bool,
    pub(crate) lan: bool,
    pub(crate) gameport: i32,
    pub(crate) specport: i32
}