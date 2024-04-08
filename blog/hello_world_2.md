---
title: Hello Again, World
url: hello_world_2
description: Oxidized my personal site
date: 04-07-2024
date_updated: 04-07-2024
public: true
---

# The Rewrite

## Again?

I decided to rebuild my site again in what may appear to be an 
annual tradition. Hopefully this will satisfy me for quite a 
while with its lack of glamour and full focus on content and 
style. Jackbranchdev2 wasn't the prettiest site in the world, 
but it looked alright. My goal with this new site is to have 
it look properly good. I want blogposts to reach actual 
readers.

I wasn't very motivated to work on my previous site and I for 
whatever reason wasn't really enjoying the workflow I had set 
up with Astro. Astro is a great framework, and it deserves the 
attention it's gotten over the past few years, but I'm simply 
not loving it right now. Instead, I'll just stick to plain 
HTML+CSS for my personal site as it needs nothing more than
that.

# The Stack

The stack for this new revision is quite simple again. It is 
exclusively Rust and Markdown. I'm using Axum+Askama+HTMX to
render and serve a responsive UI styled with TailwindCSS.

## How does it work?

When I compile the application, the `build.rs` file runs and
converts all of my markdown files to structs containing 
metadata and paths to the converted HTML. Using the 
`generation` library that is a member of this workspace, I
create a constant value that is accessible to my program at 
runtime. Each of the blogposts include their HTML content with
the `include_str!` macro.

### Code Generation

*Some code here showing the generation*

### The Server

*Some code here showing the axum server*

## Improvements to make

### Static-Site Generation

I'd like to simplify the runtime of the application by having
all of the HTML generated at compile time rather than having 
*most* of the HTML generated at compile time. Currently, all
of the HTML for the blogposts are generated at compile time, 
however, they're dynamically placed within the blogpost 
template I've written using Askama templating.

```html
<h2>{{ post.metadata.title }}</h2>
{{ post.html|escape("none") }}
```

However, I could execute these askama functions at compile time
to generate static HTML and remove the need for the
interpolation at runtime. This should reduce some amount of 
runtime complexity and allow for more flexibility with the 
server. I could for example just generate a folder of HTML 
files and serve that with NGINX or any other HTTP server I 
desire though I'm fine with my Axum server for now.
