mod input_handler;
mod scraper;

fn main() {
    let url: String = "https://anitaku.to/search.html?keyword=".to_string();
    let url_ending = input_handler::main();

    let finished_url: String = format!("{}{}", url, url_ending);

    scraper::get_anime_episodes(finished_url);
    let _ = scraper::main();
}
