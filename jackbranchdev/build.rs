use generation::{BlogPost, FrontMatter};
use gray_matter::engine::YAML;
use gray_matter::Matter;
use regex::Regex;
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

fn main() {
    // generate hashmap and vector of blogposts from
    //  blog directory
    let path = std::path::Path::new("../blog");

    let mut posts: Vec<BlogPost> = Vec::new();
    let html_output_dir = PathBuf::from("public/blog/");
    match fs::read_dir(path) {
        Ok(dir) => {
            // This deletion and deletion code is VERY temporary. It's horrible
            // see if temp exists already, if not, create it
            if !PathBuf::from("tmp.data").exists() {
                // create file
                File::create("tmp.data").unwrap();
            } else {
                // delete it if it does exist
                fs::remove_file(PathBuf::from("tmp.data")).unwrap();
                File::create("tmp.data").unwrap();
            }
            dir.map(|f| f.unwrap()).for_each(|f| {
                // read files into structs of metadata via frontmatter and
                //  generate HTML content from markdown under the frontmatter
                // read file
                match fs::read_to_string(f.path()) {
                    Ok(content) => {
                        // use gray-matter to get metadata
                        let matter = Matter::<YAML>::new();

                        let front_matter: FrontMatter = matter
                            .parse_with_struct::<FrontMatter>(&content)
                            .unwrap()
                            .data;

                        let raw_html = remove_frontmatter(&content);

                        let opts = markdown::Options {
                            parse: markdown::ParseOptions::mdx(),
                            compile: markdown::CompileOptions::gfm(),
                        };
                        let html = markdown::to_html_with_options(&raw_html, &opts).unwrap();

                        posts.push(BlogPost {
                            content,
                            front_matter,
                            html,
                        });
                    }
                    Err(e) => {
                        println!("Error occurred: {}", e);
                    }
                }
            });
        }
        Err(e) => {
            panic!(
                "ERROR OPENING PATH: {}, {}",
                path.canonicalize().unwrap().to_str().unwrap(),
                e
            );
        }
    }

    let mut data_file = OpenOptions::new()
        .write(true) // Allow writing to the file
        .append(true) // Append mode
        .open("tmp.data") // Specify the file name or path
        .expect("Failed to open file");

    // iterate through blogposts and update files
    for post in posts.iter().filter(|p| p.front_matter.public) {
        let html_file_str = format!("{}.html", post.front_matter.url);
        let mut html_path = html_output_dir.clone();
        html_path.push(PathBuf::from(html_file_str.clone()));
        // Create HTML file if not exist
        if !html_path.exists() {
            // create file
            File::create(html_path.clone()).unwrap();
        }

        data_file
            .write_fmt(format_args!(
                "title: {}, desc: {}\n",
                post.front_matter.title, post.front_matter.description
            ))
            .expect("Failed to write to file");

        // write HTML to output (proof of concept)
        let mut html_file = OpenOptions::new()
            .write(true) // Allow writing to the file
            .append(false) // Append mode off to overwrite
            .open(html_path) // Specify the file name or path
            .expect(format!("Failed to open file {}", post.front_matter.url).as_str());
        html_file
            .write(post.html.as_bytes())
            .expect("Failed to write to file");
    }
}

fn remove_frontmatter(content: &str) -> String {
    // Define a regex pattern to match frontmatter
    let frontmatter_pattern = Regex::new(r"^\s*---(.|\n)*?---").unwrap();

    // Replace frontmatter with an empty string
    let content_without_frontmatter = frontmatter_pattern.replace(content, "");

    // Convert the result to a String
    content_without_frontmatter.to_string()
}
