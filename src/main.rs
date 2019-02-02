extern crate askama;
extern crate chrono;
extern crate pulldown_cmark;

use askama::Template;

use pulldown_cmark::{Event, Parser};

use chrono::prelude::*;

use std::fs;

use std::path::Path;

#[derive(Template)]
#[template(path = "post.html", escape = "none")]

struct PostTemplate<'a> {
    content: &'a str,
    name: &'a str,
    tags: Vec<String>,
    date: &'a str,
}

fn main() {
    let posts = fs::read_dir("posts").expect("No posts directory found");
    let mut i = 0;
    if Path::new("dist").exists() {
        fs::remove_dir_all("dist").expect("Cannot clean dist directory");
    }
    fs::create_dir_all("dist/posts").expect("Cannot create dist directory");

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
        for event in parser {
            if let Event::Html(text) = event {
                if text.starts_with("<!--") {
                    let lines: Vec<&str> = text.lines().collect();
                    if lines.len() != 5 {
                        panic!(r#"Invalid metadata in post "{}""#, name);
                    }
                    name = String::from(lines[1]);
                    date = NaiveDate::parse_from_str(lines[2], "%Y-%m-%d")
                        .unwrap_or_else(|_| panic!(r#"Invalid date specifier in post "{}""#, name));
                    tags = lines[3].split(',').map(String::from).collect();
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
            name: &name,
            content: &content,
            date: &format!("{}", date.format("%Y")),
            tags: tags,
        };
        fs::write(
            format!("dist/posts/{}.html", i),
            post.render().expect("Post rendering failed"),
        )
        .expect("Cannot write to dist directory");
        i += 1;
    }
}
