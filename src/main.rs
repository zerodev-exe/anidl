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

    let body = http::get_html(temp_url.to_string()).await;

    let anime_url = scraper::get_anime_url(body.clone()).await;

    let chosen_anime = input_handler::number_parser();

    // ==============================

    let test_variable = "https://ewaezdrdab.bd36019.com/user1342/dbd107da0b94cfb05ec92bec5860c185/EP.11.v1.1718302504.1080p.mp4?token=ETfFvPWmpjxA5hPoAjzG7g&expires=1718336640&id=226919&title=(1920x1080-gogoanime)wind-breaker-episode-11.mp4".to_string();
    println!("{}", test_variable);

    let _ =
        download::download_file(test_variable, "EP.11.v1.1718302504.1080p.mp4".to_string()).await;

    // ==============================

    let anime_url_ending = anime_url[chosen_anime - 1].clone();
    println!("{}", anime_url_ending);

    let anime_name = scraper::get_anime_name(body).await;
    let path = anime_name[chosen_anime - 1].clone();
    println!("{}", path);

    let episode_vec = scraper::get_anime_episodes(anime_url_ending).await;
    for episode in episode_vec.iter() {
        println!("Episode URL: {}", episode);
    }
}
