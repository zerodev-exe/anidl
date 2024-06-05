use colored::Color::*;
use select::document::Document;
use select::predicate::{Attr, Class, Name, Predicate};

use crate::http;

static URL: &str = " https://anitaku.so";

pub fn get_anime_episodes(url: &str) -> &str {
    println!("{}", url);
    let urls = "https://5af802wiv1.bd36019.com/user1342/2afb7116ad3734b467e49b206eb380ed/EP.100.v0.1639245168.720p.mp4?token=6cvIhK61KeUk5Aj5QsGsOw&expires=1715818860&id=107342";
    urls
}

pub async fn get_anime_url(url: &str) -> Vec<String> {
    let body = http::get_html(url.to_string()).await;

    let mut anime_list: Vec<_> = vec![];
    let mut number_list = 0;

    let document = Document::from(body.as_str());
    for node in document.find(Attr("class", "name").descendant(Name("a"))) {
        number_list += 1;
        println!("[{}]  ({})", number_list, node.text());
        anime_list.push(format!("{}{}", URL, node.attr("href").unwrap()))
    }

    anime_list
}
