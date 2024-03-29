use serde::Deserialize;

#[derive(Deserialize)]
pub struct FrontMatter {
    pub title: String,
    pub description: String,
    //pub date: String,
    //pub date_updated: String,
    //pub public: bool
}
