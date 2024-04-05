use serde::Deserialize;

#[derive(Deserialize)]
pub struct FrontMatter {
    pub title: String,
    pub description: String,
    pub url: String,
    pub date: String,
    pub date_updated: String,
    pub public: bool,
}

pub struct BlogPost {
    pub front_matter: FrontMatter,
    pub content: String,
    pub html: String,
}
