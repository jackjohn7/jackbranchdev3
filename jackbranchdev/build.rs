use generation::FrontMatter;
use gray_matter::engine::YAML;
use gray_matter::Matter;
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

fn main() {
    // generate hashmap and vector of blogposts from
    //  blog directory
    let path = std::path::Path::new("../blog");
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

                        let front: FrontMatter = matter
                            .parse_with_struct::<FrontMatter>(&content)
                            .unwrap()
                            .data;

                        // write metadata to tempfile (proof of concept)
                        let mut file = OpenOptions::new()
                            .write(true) // Allow writing to the file
                            .append(true) // Append mode
                            .open("tmp.data") // Specify the file name or path
                            .expect("Failed to open file");
                        file.write_fmt(format_args!(
                            "title: {}, desc: {}\n",
                            front.title, front.description
                        ))
                        .expect("Failed to write to file");
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
}
