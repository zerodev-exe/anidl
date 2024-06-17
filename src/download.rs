use std::fs::File;
use std::fs;

use crate::print_handleing::*;

async fn create_dir_and_file(full_path: &str, full_file_path: &str) {
    fs::create_dir_all(full_path).unwrap();
    File::create(full_file_path).unwrap();
}

async fn download_content(client: &reqwest::Client, url: &str, full_file_path: &str) -> Result<(), reqwest::Error> {
    let response = client.get(url).send().await?;
    let content = response.bytes().await?;
    tokio::fs::write(full_file_path, content).await.unwrap();
    Ok(())
}

pub async fn handle_redirect_and_download(
    url: &str,
    file_path: &str,
    episode_number: u32,
) -> Result<(), reqwest::Error> {
    let downloading_string = &format!("{}{}", "Downloading episode ", episode_number);
    let downloaded_episode = &format!("{}{}", "Successfully downloaded episode ", episode_number);

    info_print(&downloading_string);
    let client = reqwest::Client::new();
    let mut current_url = url.to_string();

    let anime_episode = format!("EP-{:03}.mp4", episode_number);
    let full_file_path = format!("Anime/{}/{}", file_path, anime_episode);
    let full_path = format!("Anime/{}/", file_path);

    create_dir_and_file(&full_path, &full_file_path).await;

    let mut download_attempts = 0;
    const MAX_DOWNLOAD_ATTEMPTS: u32 = 3;

    loop {
        let response = client.get(&current_url).send().await?;

        if response.status() == reqwest::StatusCode::FOUND {
            if let Some(location) = response.headers().get(reqwest::header::LOCATION) {
                current_url = location.to_str().unwrap().to_string();
                continue;
            }
        }

        download_content(&client, &current_url, &full_file_path).await?;

        let file_size = fs::metadata(&full_file_path).unwrap().len();
        if file_size > 0 {
            success_print(&downloaded_episode);
            return Ok(());
        } else if download_attempts < MAX_DOWNLOAD_ATTEMPTS {
            error_print("Downloaded file is empty, retrying...");
            download_attempts += 1;
            continue;
        } else {
            error_print(&format!("Failed to download a valid file after {} attempts.", MAX_DOWNLOAD_ATTEMPTS));
            return Ok(());
        }
    }
}