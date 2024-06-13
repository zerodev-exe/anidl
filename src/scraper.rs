use colored::*;
use select::document::Document;
use select::predicate::{Attr, Name, Predicate};

use crate::http;

pub fn get_anime_episodes(anime_url_ending: &str) -> &str {
    anime_url_ending
}

pub async fn get_anime_url(url: &str) -> Vec<String> {
    let mut anime_list: Vec<_> = vec![];
    let mut number_list = 0;

    let body = http::get_html(url.to_string()).await;

    let document = Document::from(body.as_str());
    for node in document.find(Attr("class", "name").descendant(Name("a"))) {
        number_list += 1;
        let num = format!("[{}]", number_list);
        println!("{} - {}", num.red(), node.text());
        let temp_val = node.attr("href").unwrap().split("/");
        let anime_url_ending = temp_val.last().expect("Someting went wrong");
        println!("{}", anime_url_ending);
        anime_list.push(anime_url_ending.to_string())
    }

    anime_list
}

pub async fn get_anime_name(body: String) -> Vec<String> {
    let mut anime_list: Vec<_> = vec![];
    let mut number_list = 0;

    let document = Document::from(body.as_str());
    for node in document.find(Attr("class", "name").descendant(Name("a"))) {
        number_list += 1;
        let num = format!("[{}]", number_list);
        println!("{} - {}", num.red(), node.text());
        anime_list.push(node.text());
    }

    anime_list
}
