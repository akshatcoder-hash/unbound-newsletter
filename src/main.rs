use dotenv::dotenv;
use std::env;
use std::collections::HashMap;
use teloxide::prelude::*;
use teloxide::utils::command::BotCommands;
use log;
use env_logger;

mod models;
mod stories;
mod utils;

use crate::stories::load_stories;
use crate::utils::send_story;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "Start the bot")]
    Start,
    #[command(description = "Choose English stories")]
    English,
    #[command(description = "Choose Hindi stories")]
    Hindi,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    let bot = Bot::new(env::var("BOT_TOKEN").expect("BOT_TOKEN not set"));

    let stories_data = match load_stories() {
        Ok(data) => data,
        Err(e) => {
            log::error!("Failed to load stories: {:?}", e);
            std::process::exit(1);
        }
    };
    let stories = stories_data.personalities;

    let language: HashMap<ChatId, String> = HashMap::new();

    teloxide::repl(bot, move |bot: Bot, msg: Message| {
        let stories = stories.clone();
        let mut language = language.clone();
        async move {
            if let Some(text) = msg.text() {
                match Command::parse(text, "unbound") {
                    Ok(Command::Start) => {
                        bot.send_message(msg.chat.id, "Welcome to Unbound! Each day, I'll share an inspiring story of a successful tech personality who took a non-traditional educational path. Please choose your preferred language using /english or /hindi.").await?;
                    }
                    Ok(Command::English) => {
                        language.insert(msg.chat.id, "english".to_string());
                        bot.send_message(msg.chat.id, "You have chosen English stories.").await?;
                        if let Some(story) = stories.first() {
                            log::info!("Sending story: {:?}", story);
                            send_story(&bot, msg.chat.id, story, "english").await?;
                        } else {
                            log::warn!("No stories found.");
                        }
                    }
                    Ok(Command::Hindi) => {
                        language.insert(msg.chat.id, "hindi".to_string());
                        bot.send_message(msg.chat.id, "You have chosen Hindi stories.").await?;
                        if let Some(story) = stories.first() {
                            log::info!("Sending story: {:?}", story);
                            send_story(&bot, msg.chat.id, story, "hindi").await?;
                        } else {
                            log::warn!("No stories found.");
                        }
                    }
                    _ => {}
                }
            }
            respond(())
        }
    })
    .await;
}