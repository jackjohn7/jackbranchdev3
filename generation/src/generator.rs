use crate::BlogPost;

pub fn generate_file_str(posts: Vec<BlogPost>) -> String {
    format!("pub const POSTS: [BlogPost; {}] = {:?};", posts.len(), posts)
}
