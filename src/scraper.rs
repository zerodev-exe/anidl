use colored::*;
use reqwest::cookie::Jar;
use reqwest::Client;
use select::document::Document;
use select::predicate::{Attr, Name, Predicate};
use std::sync::Arc;

use crate::{download, http};

static URL: &str = " https://anitaku.so";

pub async fn get_anime_episodes(anime_url_ending: String, path: &str) -> Vec<String> {
    let cookie_store = Arc::new(Jar::default());
    let client = Client::builder()
        .cookie_store(true)
        .cookie_provider(cookie_store.clone())
        .build()
        .unwrap();

    // Make a request to the login page to get initial cookies
    let response = client
        .get("https://anitaku.so/login.html")
        .send()
        .await
        .unwrap();

    // Extract CSRF token from the login page
    let login_page = client
        .get("https://anitaku.so/login.html")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    let document = scraper::Html::parse_document(&login_page);
    let selector = scraper::Selector::parse("meta[name='csrf-token']").unwrap();
    let csrf_token = document
        .select(&selector)
        .next()
        .and_then(|element| element.value().attr("content"))
        .ok_or("CSRF token not found");

    // Send credentials along with the CSRF token to log in and get a session cookie
    let response = client
        .post("https://anitaku.so/login.html")
        .form(&[
            ("email", "zerodev.exe@proton.me"),
            ("password", "Cacaman18"),
            ("_csrf", csrf_token.unwrap()),
        ])
        .send()
        .await
        .unwrap();

    let mut episode_number:u32 = 1;
    let episode_string = "episode";
    let mut episode_vec: Vec<String> = vec![];

    loop {
        let episode_url = format!(
            "{}/{}-{}-{}",
            URL, anime_url_ending, episode_string, episode_number
        );

        let response = reqwest::get(&episode_url).await.unwrap();
        if response.status() != reqwest::StatusCode::OK {
            break;
        }

        // Now you can make authenticated requests
        let authenticated_content = client
            .get(&episode_url)
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        let video_urls = get_video_url(authenticated_content).await;
        let encoded_url = video_urls.last().unwrap();
        println!("Encoded URL: {}", encoded_url);

        // let video_url = http::handle_redirect(&encoded_url).await.unwrap();
        let video_url = http::handle_redirect_and_download(&encoded_url, path, episode_number).await.unwrap();
        // download::download_file(video_url.clone(), &path, episode_number).await;
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

pub async fn get_video_url(body: String) -> Vec<String> {
    let mut anime_list: Vec<_> = vec![];

    let document = Document::from(body.as_str());
    for node in document.find(Attr("class", "cf-download").descendant(Name("a"))) {
        anime_list.push(node.attr("href").unwrap().to_string());
    }

    anime_list
}
