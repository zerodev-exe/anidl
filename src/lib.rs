pub mod download;
pub mod input_handler;
pub mod parser;
pub mod scraper;
pub mod utils;

pub static URL: &str = "https://anitaku.pe/";
pub static SEACH_URL: &str = "https://anitaku.pe/search.html?keyword=";
pub static CAT_URL: &str = "https://anitaku.pe/category/";

/// Get the anime list by name
///
/// # Arguments
///
/// * `anime_name` - The name of the anime
///
/// # Returns
///
/// A tuple containing the anime URL and the anime name
pub async fn get_anime_list_by_name(anime_name: String) -> (Vec<String>, Vec<String>, Vec<String>) {
    let trimmed_name = input_handler::trim(anime_name);
    let url = format!("{}{}", SEACH_URL, trimmed_name);
    let body = utils::get_html(url)
        .await
        .expect("An error has occured, please verify if you are connected to the internet");
    get_anime_info(body)
}

pub fn get_anime_info(body: String) -> (Vec<String>, Vec<String>, Vec<String>) {
    let anime_url = parser::get_anime_url(body.clone());
    let anime_name = parser::get_anime_name(body.clone());
    let anime_images = parser::get_anime_images(body);
    (anime_url, anime_name, anime_images)
}

pub async fn download_anime_episodes(
    anime_url_ending: String,
    path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    scraper::get_anime_episodes_and_download_the_episodes(anime_url_ending, path).await
}

pub fn get_anime_details(body: String) -> (Vec<String>, Vec<String>) {
    let anime_url = parser::get_anime_url(body.clone());
    let anime_name = parser::get_anime_name(body);
    (anime_url, anime_name)
}
