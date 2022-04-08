use reqwest;
use serde::Deserialize;
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use std::env;
use std::result::Result;

const COMMAND: &str = "!kanye";
const API_URL: &str = "https://api.kanye.rest";
const MESSAGE_INTRO: &str = "Kanye once said: ";

#[derive(Debug, Deserialize)]
struct Quote {
    quote: String,
}
struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == COMMAND {
            let message_intro: String = String::from(MESSAGE_INTRO);
            let quote = match get_quote(API_URL).await {
                Ok(data) => data.quote,
                Err(_e) => String::from("I guess we'll never know."),
            };

            let message = message_intro.clone() + &quote;

            if let Err(e) = msg.channel_id.say(&ctx.http, message).await {
                println!("Error sending message: {:?}", e);
            }
        };
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

async fn get_quote(url: &str) -> Result<Quote, &'static str> {
    let body = reqwest::get(url)
        .await
        .expect("Error: couldn't fetch the quote")
        .json::<Quote>()
        .await
        .expect("Error: couldn't serialize the quote fetched from {API_URL}");

    Ok(body)
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .await
        .expect("Failed to create a client");

    if let Err(e) = client.start().await {
        println!("Client error: {:?}", e);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic]

    async fn get_with_bad_url() {
        let _quote = match get_quote("bad.url").await {
            Ok(data) => data.quote,
            Err(_e) => String::from("I guess we'll never know."),
        };
    }
}
