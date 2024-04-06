use serde::Deserialize;

// modules
pub mod generator;

#[derive(Deserialize, Debug, Clone)]
pub struct FrontMatter {
    pub title: String,
    pub description: String,
    pub url: String,
    pub date: String,
    pub date_updated: String,
    pub public: bool,
}

#[derive(Debug, Clone)]
pub struct BlogPost {
    pub front_matter: FrontMatter,
    pub html: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Metadata {
    pub title: &'static str,
    pub description: &'static str,
    pub url: &'static str,
    pub date: &'static str,
    pub date_updated: &'static str,
    pub public: bool,
}

#[derive(Debug, Clone)]
pub struct BlogPostConst {
    pub metadata: Metadata,
    pub html: &'static str,
}
