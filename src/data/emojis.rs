use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Emoji {
    pub emoji: String,
    pub hexcode: String,
    pub group: String,
    pub annotation: String,
    #[serde(default)]
    pub tags: String,
}

pub const CATEGORIES: &[(&str, &str)] = &[
    ("all", "All"),
    ("smileys-emotion", "Smileys"),
    ("people-body", "People"),
    ("animals-nature", "Nature"),
    ("food-drink", "Food"),
    ("travel-places", "Travel"),
    ("activities", "Activities"),
    ("objects", "Objects"),
    ("symbols", "Symbols"),
    ("flags", "Flags"),
];

const OPENMOJI_URL: &str = "https://raw.githubusercontent.com/hfg-gmuend/openmoji/master/data/openmoji.json";

pub async fn fetch_emojis() -> Vec<Emoji> {
    match gloo_net::http::Request::get(OPENMOJI_URL).send().await {
        Ok(response) => {
            match response.json::<Vec<Emoji>>().await {
                Ok(all) => {
                    all.into_iter()
                        .filter(|e| !e.annotation.contains("skin tone"))
                        .collect()
                }
                Err(_) => vec![]
            }
        }
        Err(_) => vec![]
    }
}
