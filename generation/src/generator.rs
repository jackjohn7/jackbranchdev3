use crate::BlogPost;

pub fn generate_file_str(posts: Vec<BlogPost>) -> String {
    format!(
        "use crate::{{BlogPost, FrontMatter}};\npub const POSTS: [BlogPost; {}] = {:?};",
        posts.len(),
        posts
    )
}

fn format_string_value(value: String) -> String {
    format!("\"{}\"", value)
}

pub fn generate(posts: Vec<BlogPost>) -> String {
    let mut result = format!(
        "use generation::{{BlogPostConst, Metadata}};\npub const POSTS: [BlogPostConst; {}] = [",
        posts.len()
    );

    let mut idx = 0;
    let posts_len = posts.len();
    for post in posts {
        result.push_str(
            &format!(
            "BlogPostConst {{\
                metadata: Metadata {{ title: {}, description: {}, url: {}, date: {}, date_updated: {}, public: {} }},\
                html: include_str!(\"../public/blog/{}.html\") }}{}",
            format_string_value(post.front_matter.title),
            format_string_value(post.front_matter.description),
            format_string_value(post.front_matter.url.clone()),
            format_string_value(post.front_matter.date),
            format_string_value(post.front_matter.date_updated),
            post.front_matter.public,
            post.front_matter.url,
            if idx != posts_len-1 {
            ", "
            } else {
            " "
            }
        ));
        idx += 1;
    }
    result.push_str("];");

    result
}
