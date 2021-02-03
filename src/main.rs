use std::sync::mpsc::channel;
use twitch_chat_wrapper::*;

fn channel_to_join() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let channel = get_env_var("NVIM_TWITCH_CHANNEL")?;
    Ok(vec![channel])
}

fn get_env_var(key: &str) -> Result<String, Box<dyn std::error::Error>> {
    let my_var = std::env::var(key)?;
    Ok(my_var)
}

fn main() {
    let (tx, rx) = channel::<String>();
    let (tx2, rx2) = channel::<ChatMessage>();

    let twitch_name = get_env_var("NVIM_TWITCH_NAME").unwrap();
    let twitch_token = get_env_var("NVIM_TWITCH_TOKEN").unwrap();
    let channel_to_join = channel_to_join().unwrap();

    std::thread::spawn(move || run(twitch_name, twitch_token, channel_to_join, rx, tx2).unwrap());
    std::thread::spawn(move || loop {
        let _ = rx2.recv();
    });

    tx.send(std::env::args().skip(1).collect::<Vec<_>>().join(" ")).unwrap();
    std::thread::sleep(std::time::Duration::from_secs(2));
}
