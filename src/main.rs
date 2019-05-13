#![feature(inner_deref)]

extern crate askama;
extern crate base64;
extern crate chrono;
extern crate html_minifier;
extern crate mime_guess;
extern crate pulldown_cmark;

use html_minifier::HTMLMinifier;

use askama::Template;

use pulldown_cmark::{Event, Parser, Tag};

use chrono::prelude::*;

use std::fs;

use std::path::Path;

use mime_guess::get_mime_type;

use std::borrow::Cow;

#[derive(Template, Clone)]
#[template(path = "post.html", escape = "none")]

struct PostTemplate {
    content: String,
    name: String,
    tags: Vec<String>,
    date: String,
    summary: Vec<String>,
}

#[derive(Template)]
#[template(path = "posts.html")]

struct PostsTemplate {
    posts: Vec<PostTemplate>,
}

fn main() {
    let posts = fs::read_dir("posts").expect("No posts directory found");
    if Path::new("dist").exists() {
        fs::remove_dir_all("dist").expect("Cannot clean dist directory");
    }
    fs::create_dir_all("dist/posts").expect("Cannot create dist directory");
    let mut compiled_posts: Vec<PostTemplate> = Vec::new();
    let mut featured: Option<PostTemplate> = None;

    let mut article_names = Vec::new();
    for post in posts {
        let path = post.expect("Failed to parse a post's path").path();
        let markdown = &fs::read_to_string(&path).unwrap_or_else(|_| {
            panic!(
                "Something went wrong while reading the post in file {}",
                path.to_str().unwrap()
            )
        });
        let parser = Parser::new(markdown);
        let mut name: String = String::from("");
        let mut date: NaiveDateTime =
            NaiveDateTime::parse_from_str("2001-07-08T00:34:60.026490+09:30", "%+").unwrap();
        let mut tags: Vec<String> = vec![];
        let mut summary: Vec<String> = vec![];
        let mut f = false;
        for event in parser {
            if let Event::Html(text) = event {
                if text.starts_with("<!--") {
                    let lines: Vec<&str> = text.lines().collect();
                    if lines.len() < 6 {
                        panic!(r#"Invalid metadata in post "{}""#, name);
                    }
                    name = String::from(lines[1]);
                    if article_names.contains(&name) {
                        panic!(r#"Two or more articles with the name "{}" found"#, name);
                    }
                    article_names.push(name.clone());
                    date = NaiveDateTime::parse_from_str(lines[2], "%Y-%m-%d %H:%M")
                        .unwrap_or_else(|_| panic!(r#"Invalid date specifier in post "{}""#, name));
                    tags = lines[3].split(',').map(String::from).collect();
                    if tags[0] == "featured" {
                        f = true;
                        tags = tags[1..].to_vec();
                    }
                    summary = lines[4..lines.len() - 1]
                        .iter()
                        .map(|s| String::from(*s))
                        .collect();
                }
            }
        }
        let parser = Parser::new(markdown)
            .filter(|event| match event {
                Event::Html(text) => !text.starts_with("<!--"),
                _ => true,
            })
            .map(|event| match event {
                Event::Start(Tag::Image(url, alt)) => {
                    let ext = url.split('.').collect::<Vec<&str>>()[1];
                    let image =
                        &fs::read(&Path::new(&format!("images/{}", url))).unwrap_or_else(|_| {
                            panic!("Could not read image {}", url);
                        });
                    let data = base64::encode(image);
                    let uri = format!("data:{};base64,{}", get_mime_type(ext), data);
                    Event::Start(Tag::Image(Cow::from(uri), alt))
                }
                _ => event,
            });
        let mut content = String::new();
        pulldown_cmark::html::push_html(&mut content, parser);
        let post = PostTemplate {
            name: name.clone(),
            content,
            date: format!("{}", date.format("%B %e %Y")),
            tags,
            summary,
        };
        let mut html_minifier = HTMLMinifier::new();
        html_minifier
            .digest(post.render().expect("Post rendering failed"))
            .expect("Minifying index failed");
        fs::write(
            &Path::new(&format!("dist/posts/{}.html", name)),
            html_minifier.get_html(),
        )
        .expect("Cannot write to dist directory");
        if f {
            featured = Some(post.clone());
        }
        compiled_posts.push(post);
    }
    compiled_posts.sort_by(|a, b| a.date.cmp(&b.date));
    let posts = PostsTemplate {
        posts: compiled_posts,
    };
    let mut html_minifier = HTMLMinifier::new();
    html_minifier
        .digest(
            posts
                .render()
                .expect("Aggregated posts page rendering failed"),
        )
        .expect("Minifying aggregated posts page failed");
    fs::write(Path::new("dist/posts.html"), html_minifier.get_html())
        .expect("Cannot write to dist directory");
}
