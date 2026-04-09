use colored::*;
use gogoanime_scraper::{allanime, input_handler};
mod print_handleing;
mod scrapertui;
use print_handleing::*;
use std::process::exit;

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    let url_ending = get_url_ending(args);
    let (anime_url, anime_name) = get_anime_details(url_ending.clone()).await;

    validate_anime_url(anime_url.clone(), &url_ending);

    print_anime_list(&anime_name);

    let (chosen_anime, path) = get_chosen_anime(&anime_name);
    let anime_url_ending = get_anime_url_ending(anime_url, chosen_anime);

    // ======================= Printing the amount of episodes

    get_warning(anime_url_ending.clone()).await;

    // ======================= Printing the amount of episodes

    match scrapertui::get_anime_episodes_and_download_the_episodes(anime_url_ending, &path).await {
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

async fn get_anime_details(query: String) -> (Vec<String>, Vec<String>) {
    let cleaned = query.replace("%20", " ");
    let results = allanime::search_anime(&cleaned, "sub")
        .await
        .unwrap_or_default();
    let mut ids = Vec::new();
    let mut names = Vec::new();
    for result in results {
        ids.push(result.id);
        names.push(result.name);
    }
    (ids, names)
}

fn validate_anime_url(anime_url: Vec<String>, url_ending: &str) {
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

async fn get_warning(anime_url_ending: String) {
    let episodes = allanime::episodes_list(&anime_url_ending, "sub")
        .await
        .unwrap_or_default();
    let total_episodes = episodes.len();
    let string = format!("Downloading all {} episodes", total_episodes).yellow();
    println!("{}", string);
}
