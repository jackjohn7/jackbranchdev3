use serde::Deserialize;

// modules
pub mod generator;

#[derive(Deserialize, Debug)]
pub struct FrontMatter {
    pub title: String,
    pub description: String,
    pub url: String,
    pub date: String,
    pub date_updated: String,
    pub public: bool,
}

#[derive (Debug)]
pub struct BlogPost {
    pub front_matter: FrontMatter,
    pub html: String,
}
