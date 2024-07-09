use crate::print_handleing::*;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;
use std::fs;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

// Asynchronously creates a directory and a file at the specified paths
async fn create_dir_and_file(full_path: &str, full_file_path: &str) {
    fs::create_dir_all(full_path).expect("Couldn't create the path");
    File::create(full_file_path)
        .await
        .expect("Couldn't create the file");
}

// Asynchronously downloads content from a given URL and writes it to a file with progress bar
async fn download_content(
    client: &Client,
    url: &str,
    full_file_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut response = client.get(url).send().await?;
    let total_size = response
        .content_length()
        .ok_or("Failed to get content length")?;
    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")?
        .progress_chars("#>-"));

    // Create a temporary file path
    let temp_file_path = format!("{}.tmp", full_file_path);
    let mut file = File::create(&temp_file_path).await?;
    let mut downloaded: u64 = 0;

    while let Some(chunk) = response.chunk().await? {
        file.write_all(&chunk).await?;
        let new = std::cmp::min(downloaded + (chunk.len() as u64), total_size);
        pb.set_position(new);
        downloaded = new;
    }

    pb.finish_with_message("Download complete");

    // Rename the temporary file to the actual file path
    tokio::fs::rename(&temp_file_path, full_file_path).await?;

    Ok(())
}

// Handles redirection and downloading of content from a URL, with retry logic for failed downloads
pub async fn handle_redirect_and_download(
    encoded_url: &str,
    file_path: &str,
    episode_number: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder()
        .redirect(reqwest::redirect::Policy::limited(5)) // Limit the number of redirects to 5
        .build()?;
    let mut current_url = encoded_url.to_string();

    let anime_episode = format!("EP-{:03}.mp4", episode_number);
    let full_file_path = format!("Anime/{}/{}", file_path, anime_episode);
    let full_path = format!("Anime/{}/", file_path);

    create_dir_and_file(&full_path, &full_file_path).await;

    loop {
        println!("Handling Redirect and Download...");
        println!("Episode: {}", episode_number);
        println!("File Path: {}", file_path);
        println!("Encoded URL: {}", encoded_url);

        let response = match client.get(&current_url).send().await {
            Ok(resp) => resp,
            Err(e) => {
                error_print(&format!("Link is unreachable :(\n{}", e));
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "HTTP request failed",
                )));
            }
        };

        if response.status().is_redirection() {
            if let Some(location) = response.headers().get(reqwest::header::LOCATION) {
                let new_url = location.to_str().unwrap().to_string();
                println!("Redirecting to: {}", new_url);
                current_url = new_url;
                continue;
            }
        } else if response.status().is_client_error() {
            error_print("Too many redirects encountered.");
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "HTTP request failed",
            )));
        }

        download_content(&client, &current_url, &full_file_path).await?;

        let file_size = fs::metadata(&full_file_path).unwrap().len();
        if file_size > 0 {
            return Ok(());
        } else {
            error_print("Downloaded file is empty, retrying...");
        }
    }
}
