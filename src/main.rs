mod download;
mod http;
mod input_handler;
mod scraper;

static URL: &str = " https://anitaku.so";

#[tokio::main]
async fn main() {
    // let url_ending = input_handler::main();
    // let scraper_url_base: String = format!("{}{}", URL, url_ending);

    let temp_url = "https://anitaku.so/search.html?keyword=Tokyo%20ghoul";

    // let scraped = scraper::get_anime_episodes(&temp_url);

    // anime_episodes.push(scraper::get_anime_episodes(URL));

    let anime_url = scraper::get_anime_url(temp_url).await;

    let chosen_anime = input_handler::number_parser();

    let anime_url_ending = anime_url[chosen_anime - 1].clone();
    println!("{}", anime_url_ending);
}
