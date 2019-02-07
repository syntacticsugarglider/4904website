#![feature(inner_deref)]

extern crate askama;
extern crate base64;
extern crate chrono;
extern crate html_minifier;
extern crate ical;
extern crate mime_guess;
extern crate pulldown_cmark;
extern crate reqwest;

use html_minifier::HTMLMinifier;

use askama::Template;

use pulldown_cmark::{Event, Parser, Tag};

use chrono::prelude::*;

use std::fs;

use std::path::Path;

use mime_guess::get_mime_type;

use std::borrow::Cow;

use ical::IcalParser;

use std::collections::HashMap;

#[derive(Template, Clone)]
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
#[template(path = "posts.html")]

struct PostsTemplate {
    posts: Vec<PostTemplate>,
}

#[derive(Template)]
#[template(path = "index.html")]

struct IndexTemplate<'a> {
    featured_post: Option<PostTemplate>,
    events: Vec<(&'a NaiveDate, &'a Vec<CalendarEvent>)>,
}

#[derive(Clone)]

struct CalendarEvent {
    start_date: NaiveDate,
    end_date: Option<NaiveDate>,
    name: String,
}

fn main() {
    let mut resp = reqwest::get("http://calendar.google.com/calendar/ical/nuevaschool.org_mjs52qjv4sg4el7d0lb48ong4g%40group.calendar.google.com/public/basic.ics").expect("Cannot fetch calendar");
    let text = resp.text().expect("Cannot parse returned ics");
    let buffer = text.as_bytes();
    let mut parser = IcalParser::new(buffer);
    let calendar = parser
        .next()
        .expect("Cannot read calendar")
        .expect("Cannot read calendar");
    let mut events: HashMap<NaiveDate, Vec<CalendarEvent>> = HashMap::new();
    for event in calendar.events {
        let mut start_date: Option<NaiveDate> = None;
        let mut end_date: Option<NaiveDate> = None;
        let mut name: Option<String> = None;
        for property in event.properties {
            if property.name == "SUMMARY" {
                name = Some(String::from(
                    property.value.deref().expect("No value for name in event"),
                ));
            }
            if property.name == "DTSTART" {
                start_date = Some(
                    NaiveDate::parse_from_str(
                        &property
                            .value
                            .deref()
                            .expect("No value for start date in event")[0..8],
                        "%Y%m%d",
                    )
                    .expect("Cannot parse start date in event"),
                );
            }
            if property.name == "DTEND" {
                end_date = Some(
                    NaiveDate::parse_from_str(
                        &property
                            .value
                            .deref()
                            .expect("No value for end date in event")[0..8],
                        "%Y%m%d",
                    )
                    .expect("Cannot parse end date in event"),
                );
            }
        }
        let start_date = start_date.expect("No start date in event");
        events.insert(
            start_date,
            match events.get(&start_date) {
                None => vec![CalendarEvent {
                    name: name.expect("No name in event").clone(),
                    start_date: start_date,
                    end_date: end_date,
                }],
                Some(events) => {
                    let mut events: Vec<CalendarEvent> = events.to_vec();
                    events.extend(vec![CalendarEvent {
                        name: name.expect("No name in event").clone(),
                        start_date: start_date,
                        end_date: end_date,
                    }]);
                    events
                }
            },
        );
    }
    let posts = fs::read_dir("posts").expect("No posts directory found");
    let mut i = 0;
    if Path::new("dist").exists() {
        fs::remove_dir_all("dist").expect("Cannot clean dist directory");
    }
    fs::create_dir_all("dist/posts").expect("Cannot create dist directory");
    let mut compiled_posts: Vec<PostTemplate> = vec![];
    let mut featured: Option<PostTemplate> = None;

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
        let mut f = false;
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
        fs::write(
            &Path::new(&format!("dist/posts/{}.html", i)),
            html_minifier.get_html(),
        )
        .expect("Cannot write to dist directory");
        if f {
            featured = Some(post.clone());
        }
        compiled_posts.push(post);
        i += 1;
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
    let mut events: Vec<(&NaiveDate, &Vec<CalendarEvent>)> = events.iter().collect();
    events.sort_by(|a, b| a.0.cmp(&b.0));
    let index = IndexTemplate {
        featured_post: featured,
        events,
    };
    let mut html_minifier = HTMLMinifier::new();
    html_minifier
        .digest(index.render().expect("Index rendering failed"))
        .expect("Minifying index failed");
    fs::write(Path::new("dist/index.html"), html_minifier.get_html())
        .expect("Cannot write to dist directory");
}
