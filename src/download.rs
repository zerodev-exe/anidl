use crate::print_handleing::*;
use std::fs;
use std::fs::File;

// Asynchronously creates a directory and a file at the specified paths
async fn create_dir_and_file(full_path: &str, full_file_path: &str) {
    fs::create_dir_all(full_path).unwrap();
    File::create(full_file_path).unwrap();
}

// Asynchronously downloads content from a given URL and writes it to a file
async fn download_content(
    client: &reqwest::Client,
    url: &str,
    full_file_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let response = client
        .get(url)
        .send()
        .await
        .map_err(|e| format!("Failed to send request: {}", e))?;

    let content = response.bytes().await?;
    tokio::fs::write(full_file_path, content)
        .await
        .expect("Couldn't write to the file");
    Ok(())
}

// Handles redirection and downloading of content from a URL, with retry logic for failed downloads
pub async fn handle_redirect_and_download(
    url: &str,
    file_path: &str,
    episode_number: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    info_print(&format!("Downloading episode {}", episode_number));
    let client = reqwest::Client::new();
    let mut current_url = url.to_string();

    let anime_episode = format!("EP-{:03}.mp4", episode_number);
    let full_file_path = format!("Anime/{}/{}", file_path, anime_episode);
    let full_path = format!("Anime/{}/", file_path);

    create_dir_and_file(&full_path, &full_file_path).await;

    let mut download_attempts = 0;
    const MAX_DOWNLOAD_ATTEMPTS: u32 = 3;

    loop {
        let response = match client.get(&current_url).send().await {
            Ok(resp) => resp,
            Err(e) => {
                error_print(&format!("Failed to send request: {}", e));
                if download_attempts < MAX_DOWNLOAD_ATTEMPTS {
                    download_attempts += 1;
                    continue;
                } else {
                    error_print(&format!(
                        "Failed to download a valid file after {} attempts.",
                        MAX_DOWNLOAD_ATTEMPTS
                    ));
                    break;
                }
            }
        };

        // If the response indicates a redirection, update the current URL and retry
        if response.status() == reqwest::StatusCode::FOUND {
            if let Some(location) = response.headers().get(reqwest::header::LOCATION) {
                current_url = location.to_str().unwrap().to_string();
                continue;
            }
        }

        // Attempt to download the content
        download_content(&client, &current_url, &full_file_path).await?;

        // Check if the downloaded file is valid (non-empty)
        let file_size = fs::metadata(&full_file_path).unwrap().len();
        if file_size > 0 {
            success_print(&format!(
                "Successfully downloaded episode {}",
                episode_number
            ));
            return Ok(());
        } else if download_attempts < MAX_DOWNLOAD_ATTEMPTS {
            error_print("Downloaded file is empty, retrying...");
            download_attempts += 1;
            continue;
        } else {
            // If maximum download attempts are reached, log an error and retry one last time
            error_print(&format!(
                "Failed to download a valid file after {} attempts.",
                MAX_DOWNLOAD_ATTEMPTS
            ));
            break;
        }
    }
    Ok(())
}
