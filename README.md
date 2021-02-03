# Twitch Send

Command Line Tool to send messages to your twitch chat

## Prerequesites

- cargo

## Setup twitch authentication

You will need to set the following environment variables
- NVIM_TWITCH_TOKEN can be generated at [twitchapps.com/tmi](https://twitchapps.com/tmi).
- NVIM_TWITCH_NAME is your twitch name
- NVIM_TWITCH_CHANNEL the twitch channel to join

## Install the binary

```bash
git clone https://github.com/hardliner66/twitch-send
cd twitch-send
cargo install --force --path .
```

## Run
```bash
twitch-send Hello Twitch!
```
