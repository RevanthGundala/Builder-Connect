use std::env;
use serde::{Deserialize, Serialize};
use oauth2::{
    AuthorizationCode,
    AuthUrl,
    ClientId,
    ClientSecret,
    CsrfToken,
    RedirectUrl,
    Scope,
    TokenResponse,
    TokenUrl,
    basic::BasicClient,
};
use actix_web::web::Data;

fn get_oauth_variables(client_id: &str, client_secret: &str, auth_url: &str, token_url: &str, redirect_url: &str) -> [String; 5]{
    let client_id = match env::var(client_id) {
        Ok(v) => v.to_string(),
        Err(_) => format!("Error loading env variable"),
    };
    let client_secret = match env::var(client_secret) {
        Ok(v) => v.to_string(),
        Err(_) => format!("Error loading env variable"),
    };
    let auth_url = match env::var(auth_url) {
        Ok(v) => v.to_string(),
        Err(_) => format!("Error loading env variable"),
    };
    let token_url = match env::var(token_url) {
        Ok(v) => v.to_string(),
        Err(_) => format!("Error loading env variable"),
    };
    let redirect_url = match env::var(redirect_url) {
        Ok(v) => v.to_string(),
        Err(_) => format!("Error loading env variable"),
    };

    [client_id, client_secret, auth_url, token_url, redirect_url]
}

fn get_google_ouath_variables() -> [String; 5]{
    get_oauth_variables(
        "GOOGLE_OAUTH_CLIENT_ID", 
        "GOOGLE_OAUTH_CLIENT_SECRET", 
        "GOOGLE_OAUTH_AUTH_URL", 
        "GOOGLE_OAUTH_TOKEN_URL",
        "GOOGLE_OAUTH_REDIRECT_URL")
}

fn get_discord_oauth_variables() -> [String; 5]{
    get_oauth_variables("DISCORD_OAUTH_CLIENT_ID", 
    "DISCORD_OAUTH_CLIENT_SECRET", 
    "DISCORD_OAUTH_AUTH_URL", 
    "DISCORD_OAUTH_TOKEN_URL", 
    "DISCORD_OAUTH_REDIRECT_URL")
}

type OAuthClient = BasicClient;
type OAuthClientData = Data<OAuthClient>;

pub trait ClientData {
    fn new_client_data(oauth_variables: [String; 5]) -> Data<OAuthClient>;
}

impl ClientData for OAuthClient {
    fn new_client_data(oauth_variables: [String; 5]) -> Data<OAuthClient> {
        let [client_id, client_secret, auth_url, token_url, redirect_url] = oauth_variables;
        let client = BasicClient::new(
            ClientId::new(client_id),
            Some(ClientSecret::new(client_secret)),
            AuthUrl::new(auth_url).unwrap(),
            Some(TokenUrl::new(token_url).unwrap()))
            .set_redirect_uri(RedirectUrl::new(redirect_url).unwrap());
        let oauth_client_data = Data::new(client);
        oauth_client_data
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")] 
#[serde(tag = "client_type", content = "value")]
pub enum ClientType{
    Google,
    Discord
}

pub fn get_client_data(client_types: Vec<ClientType>) -> Vec<OAuthClientData> {
    let mut client_data: Vec<OAuthClientData> = vec![];
    client_types 
        .iter()
        .for_each(|client_type| {
            let oauth_client_data = match client_type {
                ClientType::Google => OAuthClient::new_client_data(get_google_ouath_variables()),
                ClientType::Discord => OAuthClient::new_client_data(get_discord_oauth_variables())
            };
            client_data.push(oauth_client_data);
        });
    client_data
}