use colored::*;
use select::document::Document;
use select::predicate::{Attr, Name, Predicate};

static URL: &str = " https://anitaku.so";

pub async fn get_anime_episodes(anime_url_ending: String) -> Vec<String> {
    let mut episode_number = 1;
    let episode_string = "episode";
    let mut episode_vec:Vec<String> = vec![];

    loop {
        let episode_url = format!(
            "{}/{}-{}-{}",
            URL, anime_url_ending, episode_string, episode_number
        );

        let response = reqwest::get(&episode_url).await.unwrap();
        if response.status() != reqwest::StatusCode::OK{
            break;
        }

        println!("{}", episode_url);
        episode_vec.push(episode_url.clone());
        episode_number += 1;
    }

    return episode_vec;

}

pub async fn get_anime_url(body: String) -> Vec<String> {
    let mut anime_list: Vec<_> = vec![];
    let mut number_list = 0;

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

    let document = Document::from(body.as_str());
    for node in document.find(Attr("class", "name").descendant(Name("a"))) {
        anime_list.push(node.text());
    }

    anime_list
}
