use twitch_irc::login::{CredentialsPair, StaticLoginCredentials};
use twitch_irc::ClientConfig;
use twitch_irc::TCPTransport;
use twitch_irc::TwitchIRCClient;

fn channel_to_join() -> Result<String, Box<dyn std::error::Error>> {
    let channel = get_env_var("TWITCH_SEND_CHANNEL")?;
    Ok(channel)
}

fn get_env_var(key: &str) -> Result<String, Box<dyn std::error::Error>> {
    let my_var = std::env::var(key)?;
    Ok(my_var)
}

#[tokio::main]
async fn main() {
    let twitch_name = get_env_var("TWITCH_SEND_NAME").unwrap();
    let twitch_token = get_env_var("TWITCH_SEND_TOKEN")
        .unwrap()
        .replacen("oauth:", "", 1);
    let channel_to_join = channel_to_join().unwrap();

    // default configuration is to join chat as anonymous.
    let config = ClientConfig {
        login_credentials: StaticLoginCredentials {
            credentials: CredentialsPair {
                login: twitch_name.clone(),
                token: Some(twitch_token),
            },
        },
        ..ClientConfig::default()
    };

    let (mut _incoming_messages, client) =
        TwitchIRCClient::<TCPTransport, StaticLoginCredentials>::new(config);

    // join a channel
    client.join(channel_to_join.clone());
    let msg = std::env::args().skip(1).collect::<Vec<_>>().join(" ");

    client.say(twitch_name, msg).await.unwrap();
}
