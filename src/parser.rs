use crate::download;
use crate::print_handleing::*;
use crate::scraper::get_video_url;

static URL: &str = "https://anitaku.so/";

lazy_static::lazy_static! {
    static ref CLIENT: reqwest::Client = reqwest::Client::builder()
        .cookie_store(true)
        .build()
        .unwrap();
}

trait HttpClient {
    async fn get(&self, url: &str) -> Result<String, Box<dyn std::error::Error>>;
    async fn post(
        &self,
        url: &str,
        form: &[(&str, &str)],
    ) -> Result<(), Box<dyn std::error::Error>>;
}

impl HttpClient for reqwest::Client {
    async fn get(&self, url: &str) -> Result<String, Box<dyn std::error::Error>> {
        let response = self.get(url).send().await?.text().await?;
        Ok(response)
    }

    async fn post(
        &self,
        url: &str,
        form: &[(&str, &str)],
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.post(url).form(form).send().await?;
        Ok(())
    }
}

async fn get_csrf_token<T: HttpClient>(client: &T) -> Result<String, Box<dyn std::error::Error>> {
    let login_page = client.get(&format!("{}{}", URL,"login.html")).await?;
    let document = scraper::Html::parse_document(&login_page);
    let selector = scraper::Selector::parse("meta[name='csrf-token']")?;
    let csrf_token = document
        .select(&selector)
        .next()
        .and_then(|element| element.value().attr("content"))
        .ok_or("CSRF token not found")?;
    Ok(csrf_token.to_string())
}

async fn login<T: HttpClient>(
    client: &T,
    csrf_token: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    client
        .post(
            "https://anitaku.so/login.html",
            &[
                ("email", "zerodev.exe@proton.me"),
                ("password", "Cacaman18"),
                ("_csrf", csrf_token),
            ],
        )
        .await?;
    Ok(())
}

pub async fn get_anime_episodes(
    anime_url_ending: String,
    path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = CLIENT.clone();

    client.get("https://anitaku.so/login.html").send().await?;
    let csrf_token = get_csrf_token(&client).await?;
    login(&client, &csrf_token).await?;

    let mut episode_number: u32 = 1;
    let episode_string = "episode";

    loop {
        let anime_episode = format!("EP-{:03}.mp4", episode_number);
        let full_file_path = format!("Anime/{}/{}", path, anime_episode);

        let path_to_file = std::path::Path::new(&full_file_path);
        if path_to_file.exists() {
            let metadata = std::fs::metadata(&full_file_path)?;
            if metadata.len() > 0 {
                success_print(&format!(
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

        let response = reqwest::get(&episode_url).await?;
        if response.status() != reqwest::StatusCode::OK {
            break;
        }

        let authenticated_content = client.get(&episode_url).send().await?.text().await?;
        let video_urls = get_video_url(authenticated_content);
        let encoded_url = video_urls.last().ok_or("No video URL found")?;

        download::handle_redirect_and_download(&encoded_url, path, episode_number).await?;

        episode_number += 1;
    }
    Ok(())
}
