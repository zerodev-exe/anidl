mod download;
mod http;
mod input_handler;
mod logik;
mod scraper;
mod print_handleing;

use crate::print_handleing::*;

static URL: &str = " https://anitaku.so/search.html?keyword=";

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    let url_ending = if args.len() > 1 {
        input_handler::trim(args[1].clone())
    } else {
        input_handler::init_input()
    };
    let scraper_url_base: String = format!("{}{}", URL, url_ending);

    let body = http::get_html(scraper_url_base.to_string())
        .await
        .expect("Your request didn't work");

    let anime_url = scraper::get_anime_url(body.clone()).await;

    let chosen_anime = input_handler::number_parser();

    let anime_url_ending = anime_url[chosen_anime - 1].clone();
    debug_print(&format!("Chosen anime: {}", anime_url_ending));

    let anime_name = scraper::get_anime_name(body).await;
    let path = anime_name[chosen_anime - 1].clone();

    logik::get_anime_episodes(anime_url_ending, &path).await;
}
