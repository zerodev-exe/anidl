mod download;
mod http;
mod input_handler;
mod parser;
mod print_handleing;
mod scraper;
use crate::print_handleing::*;
use colored::Colorize;
use std::process::exit;

pub static URL: &str = "https://anitaku.pe/";

static BASE_URL: &str = "https://anitaku.pe/search.html?keyword=";

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    let url_ending = if args.len() > 1 {
        input_handler::trim(args[1].clone())
    } else {
        input_handler::init_input()
    };

    let scraper_url_base: String = format!("{}{}", BASE_URL, url_ending);

    let body = http::get_html(scraper_url_base.to_string())
        .await
        .expect("Failed to retrieve HTML content");

    // NOTE: Printing the anime names
    // =========================================
    let mut number_list = 0;

    let anime_url = scraper::get_anime_url(body.clone());

    let anime_name = scraper::get_anime_name(body);

    if anime_url.clone().unwrap().is_empty() {
        error_print(&format!(
            "{} : {}",
            "No anime found with the name", url_ending
        ));
        exit(1)
    }

    for i in anime_name.clone() {
        number_list += 1;
        let num = format!("[{}]", number_list);
        println!("{} - {}", num.red(), i);
    }

    // =========================================

    let chosen_anime = input_handler::number_parser();
    let path = anime_name[chosen_anime - 1].clone();

    let anime_url_ending = match anime_url {
        Ok(urls) => urls[chosen_anime - 1].clone(),
        Err(e) => panic!("Error retrieving anime URL: {}", e),
    };

    debug_print(&format!("Chosen anime: {}", anime_url_ending));

    let _ = parser::get_anime_episodes_and_download_the_episodes(anime_url_ending, &path).await;
}
