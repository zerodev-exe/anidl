use crate::download;
use crate::print_handleing::*;
use crate::scraper::get_video_url;
use reqwest::cookie::Jar;
use reqwest::Client;
use std::sync::Arc;

static URL: &str = "https://anitaku.so";

pub async fn get_anime_episodes(anime_url_ending: String, path: &str) {
    let cookie_store = Arc::new(Jar::default());
    let client = Client::builder()
        .cookie_store(true)
        .cookie_provider(cookie_store.clone())
        .build()
        .expect("Failed to build client");

    // Make a request to the login page to get initial cookies
    let _initial_response = client
        .get("https://anitaku.so/login.html")
        .send()
        .await
        .expect("Failed to fetch initial login page");

    // Extract CSRF token from the login page
    let login_page_html = client
        .get("https://anitaku.so/login.html")
        .send()
        .await
        .expect("Failed to fetch login page for CSRF")
        .text()
        .await
        .expect("Failed to extract text from login page");
    let document = scraper::Html::parse_document(&login_page_html);
    let selector =
        scraper::Selector::parse("meta[name='csrf-token']").expect("Failed to parse selector");
    let csrf_token = document
        .select(&selector)
        .next()
        .and_then(|element| element.value().attr("content"))
        .expect("CSRF token not found");

    // Send credentials along with the CSRF token to log in and get a session cookie
    let _response = client
        .post("https://anitaku.so/login.html")
        .form(&[
            ("email", "zerodev.exe@proton.me"),
            ("password", "Cacaman18"),
            ("_csrf", csrf_token),
        ])
        .send()
        .await
        .unwrap();

    let mut episode_number: u32 = 1;
    let episode_string = "episode";

    loop {
        let anime_episode = format!("EP-{:03}.mp4", episode_number);
        let full_file_path = format!("Anime/{}/{}", path, anime_episode);

        let path_to_file = std::path::Path::new(&full_file_path);
        if path_to_file.exists() {
            let metadata = std::fs::metadata(&full_file_path).unwrap();
            if metadata.len() > 0 {
                info_print(&format!(
                    "File {} already exists and is not empty, skipping...",
                    full_file_path
                ));
                episode_number += 1;
                continue;
            } else {
                error_print(&format!(
                    "File {} already exists but is empty, proceeding with download...",
                    full_file_path
                ));
            }
        }

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

        let _ = download::handle_redirect_and_download(&encoded_url, path, episode_number).await;

        episode_number += 1;
    }
}
