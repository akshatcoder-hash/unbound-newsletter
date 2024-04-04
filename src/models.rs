use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Story {
    pub name: String,
    pub profile_link: String,
    pub story: StoryText,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StoryText {
    pub english: String,
    pub hindi: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StoriesData {
    pub personalities: Vec<Story>,
}
