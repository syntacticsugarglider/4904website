extern crate askama;
extern crate chrono;
extern crate html_minifier;
extern crate pulldown_cmark;

use html_minifier::HTMLMinifier;

use askama::Template;

use pulldown_cmark::{Event, Parser};

use chrono::prelude::*;

use std::fs;

use std::path::Path;

#[derive(Template)]
#[template(path = "post.html", escape = "none")]

struct PostTemplate {
    content: String,
    name: String,
    tags: Vec<String>,
    date: String,
    index: i32,
    summary: Vec<String>,
}

#[derive(Template)]
#[template(path = "posts.html", escape = "none")]

struct PostsTemplate {
    posts: Vec<PostTemplate>,
}

#[derive(Template)]
#[template(path = "index.html", escape = "none")]

struct IndexTemplate {}

fn main() {
    let posts = fs::read_dir("posts").expect("No posts directory found");
    let mut i = 0;
    if Path::new("dist").exists() {
        fs::remove_dir_all("dist").expect("Cannot clean dist directory");
    }
    fs::create_dir_all("dist/posts").expect("Cannot create dist directory");
    let mut compiled_posts: Vec<PostTemplate> = vec![];

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
        let mut date: NaiveDate = NaiveDate::from_yo(2015, 73);
        let mut tags: Vec<String> = vec![];
        let mut summary: Vec<String> = vec![];
        for event in parser {
            if let Event::Html(text) = event {
                if text.starts_with("<!--") {
                    let lines: Vec<&str> = text.lines().collect();
                    if lines.len() < 6 {
                        panic!(r#"Invalid metadata in post "{}""#, name);
                    }
                    name = String::from(lines[1]);
                    date = NaiveDate::parse_from_str(lines[2], "%Y-%m-%d")
                        .unwrap_or_else(|_| panic!(r#"Invalid date specifier in post "{}""#, name));
                    tags = lines[3].split(',').map(String::from).collect();
                    summary = lines[4..lines.len() - 1]
                        .iter()
                        .map(|s| String::from(*s))
                        .collect();
                }
            }
        }
        let parser = Parser::new(markdown).filter(|event| match event {
            Event::Html(text) => !text.starts_with("<!--"),
            _ => true,
        });
        let mut content = String::new();
        pulldown_cmark::html::push_html(&mut content, parser);
        let post = PostTemplate {
            name,
            content,
            date: format!("{}", date.format("%B %e %Y")),
            tags,
            index: i,
            summary,
        };
        let mut html_minifier = HTMLMinifier::new();
        html_minifier
            .digest(post.render().expect("Post rendering failed"))
            .expect("Minifying index failed");
        fs::write(format!("dist/posts/{}.html", i), html_minifier.get_html())
            .expect("Cannot write to dist directory");
        compiled_posts.push(post);
        i += 1;
    }
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
    fs::write("dist/posts.html", html_minifier.get_html()).expect("Cannot write to dist directory");
    let index = IndexTemplate {};
    let mut html_minifier = HTMLMinifier::new();
    html_minifier
        .digest(index.render().expect("Index rendering failed"))
        .expect("Minifying index failed");
    fs::write("dist/index.html", html_minifier.get_html()).expect("Cannot write to dist directory");
}
