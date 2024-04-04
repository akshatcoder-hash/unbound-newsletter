use teloxide::prelude::*;
use teloxide::types::ParseMode;
use teloxide::RequestError;
use crate::models::Story;
use regex::Regex;
use log;

pub async fn send_story(bot: &Bot, chat_id: ChatId, story: &Story, language: &str) -> Result<(), RequestError> {
    let story_text = match language {
        "hindi" => &story.story.hindi,
        _ => &story.story.english,
    };

    // Regex to escape MarkdownV2 special characters
    let escape_regex = Regex::new(r"([_*\[\]()~`>#+\-=|{}.!\\])").unwrap();
    let escaped_story_text = escape_regex.replace_all(story_text, r"\$1");
    let escaped_name = escape_regex.replace_all(&story.name, r"\$1");
    let escaped_profile_link = escape_regex.replace_all(&story.profile_link, r"\$1");

    let message = format!(
        "*{}*\n\n{}\n\nRead more: {}",
        escaped_name, escaped_story_text, escaped_profile_link
    );

    match bot.send_message(chat_id, &message).parse_mode(ParseMode::MarkdownV2).await {
        Ok(_) => {
            log::info!("Story sent successfully.");
            Ok(())
        },
        Err(e) => {
            log::error!("Failed to send story: {:?}", e);
            Err(e)
        }
    }
}
