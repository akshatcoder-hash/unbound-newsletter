use crate::models::StoriesData;
use std::fs;

pub fn load_stories() -> Result<StoriesData, Box<dyn std::error::Error>> {
    let stories_json = fs::read_to_string("/Users/akshatsharma/unbound-newsletter/src/data/stories.json")?;
    let stories_data: StoriesData = serde_json::from_str(&stories_json)?;
    Ok(stories_data)
}
