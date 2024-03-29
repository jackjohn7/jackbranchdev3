# jackbranchdev3

Astro is awesome. I'm kind of overwhelmed by it though. Astro just 
released their own database. The entire project is super impressive 
and it's a great technology for a content-driven site. However, my
project doesn't warrant the complexity that Astro comes with. It's a 
sick katana and I merely need a scalpel.

Because of this, I've decided to tinker around with building my site 
with Rust leveraging compile-time code-generation to maintain
type-safety and a good DX working with blogposts generated from
markdown.

In my move to Rust, I will lose a handful of features as a result of 
my own incompetence and laziness. I will not bother with reinventing 
the entire 18-wheeler that is Astro. Features such as build-time 
validation of links among others are going to be absent here. I'll 
just have to be a bit more careful.

## Why Rust?

I figured Rust would be a solid choice for its ability to have
compile-time logic. I only know of a handful of languages that have
that functionality, and none of them are as simple as Rust. My 
`build.rs` file handles all my code and HTML generation.

## Code-Generation

1. Frontmatter is read from file
2. Content separated from frontmatter is read from files
3. Content is parsed into HTML and placed in public/posts/
4. Frontmatter structs are used to generate a HashMap at 
compile-time.
5. To avoid regenerating data too eagerly, a temporary 
file is created with the hashsums of each file. This file 
is ignored by Git and is not included in compiled binary.

