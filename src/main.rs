use gogoanime_scraper::{input_handler, parser, scraper, utils, SEACH_URL};
mod print_handleing;
use print_handleing::*;
use std::process::exit;

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    let url_ending = get_url_ending(args);
    let scraper_url_base = get_scraper_url_base(&url_ending);

    let body = fetch_html_body(scraper_url_base)
        .await
        .expect("Failed to retrieve HTML content");

    let (anime_url, anime_name) = get_anime_details(body);

    validate_anime_url(&anime_url, &url_ending);

    print_anime_list(&anime_name);

    let (chosen_anime, _path) = get_chosen_anime(&anime_name);
    let anime_url_ending = get_anime_url_ending(anime_url, chosen_anime);


    match scraper::get_anime_episodes_and_download_the_episodes(anime_url_ending, &_path).await {
        Ok(_) => success_print("Successfully downloaded all of the episodes"),
        Err(_) => error_print("Failed to download all of the episodes"),
    }
}

fn get_url_ending(args: Vec<String>) -> String {
    if args.len() > 1 {
        input_handler::trim(args[1].clone())
    } else {
        input_handler::init_input()
    }
}

fn get_scraper_url_base(url_ending: &str) -> String {
    format!("{}{}", SEACH_URL, url_ending)
}

async fn fetch_html_body(scraper_url_base: String) -> Result<String, reqwest::Error> {
    utils::get_html(scraper_url_base).await
}

fn get_anime_details(body: String) -> (Vec<String>, Vec<String>) {
    let anime_url = parser::get_anime_url(body.clone());
    let anime_name = parser::get_anime_name(body);
    (anime_url, anime_name)
}

fn validate_anime_url(anime_url: &Vec<String>, url_ending: &str) {
    if anime_url.is_empty() {
        error_print(&format!("No anime found with the name: {}", url_ending));
        exit(1);
    }
}

fn get_chosen_anime(anime_name: &[String]) -> (usize, String) {
    let chosen_anime = input_handler::number_parser().unwrap();
    let path = anime_name[chosen_anime - 1].clone();
    (chosen_anime, path)
}

fn get_anime_url_ending(anime_url: Vec<String>, chosen_anime: usize) -> String {
    anime_url[chosen_anime - 1].clone()
}
