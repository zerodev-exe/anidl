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

// Handles redirection and returns the final download link from a URL, with retry logic for failed requests
pub async fn handle_redirect_and_get_link(
    encoded_url: &str,
    file_path: &str,
    episode_number: u32,
) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::builder()
        .redirect(reqwest::redirect::Policy::limited(5)) // Limit the number of redirects to 5
        .build()?;
    let mut current_url = encoded_url.to_string();

    let anime_episode = format!("EP-{:03}.mp4", episode_number);
    let full_file_path = format!("Anime/{}/{}", file_path, anime_episode);
    let full_path = format!("Anime/{}/", file_path);

    create_dir_and_file(&full_path, &full_file_path).await;

    loop {
        let response = match client.get(&current_url).send().await {
            Ok(resp) => resp,
            Err(_) => {
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
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "HTTP request failed",
            )));
        }

        // Return the final URL instead of downloading the content
        return Ok(current_url);
    }
}

// Asynchronously downloads content from a given URL and writes it to a file
async fn download_content(
    client: &Client,
    url: &str,
    full_file_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut response = client.get(url).send().await?;
    let total_size = response
        .content_length()
        .ok_or("Failed to get content length")?;

    let temp_file_path = create_temp_file_path(full_file_path);
    let mut file = File::create(&temp_file_path).await?;
    let mut downloaded: u64 = 0;

    while let Some(chunk) = response.chunk().await? {
        file.write_all(&chunk).await?;
        downloaded = std::cmp::min(downloaded + (chunk.len() as u64), total_size);
    }

    tokio::fs::rename(&temp_file_path, full_file_path).await?;
    let file_size = fs::metadata(full_file_path).unwrap().len();
    if file_size > 0 {
        Ok(())
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Download failed",
        )))
    }
}

// Handles redirection and downloads the content from the final URL
pub async fn handle_redirect_and_download(
    encoded_url: &str,
    file_path: &str,
    episode_number: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    let final_url = handle_redirect_and_get_link(encoded_url, file_path, episode_number).await?;
    let client = Client::new();
    let anime_episode = format!("EP-{:03}.mp4", episode_number);
    let full_file_path = format!("Anime/{}/{}", file_path, anime_episode);
    download_content(&client, &final_url, &full_file_path).await?;
    Ok(())
}

fn create_temp_file_path(full_file_path: &str) -> String {
    format!("{}.tmp", full_file_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::fs::remove_file;

    #[tokio::test]
    async fn test_create_dir_and_file() {
        let full_path = "test_dir";
        let full_file_path = "test_dir/test_file.txt";
        create_dir_and_file(full_path, full_file_path).await;
        assert!(std::path::Path::new(full_file_path).exists());
        remove_file(full_file_path).await.unwrap();
        std::fs::remove_dir(full_path).unwrap();
    }

    #[tokio::test]
    async fn test_download_content() {
        let client = Client::new();
        let url = "https://httpbin.org/bytes/1024";
        let full_file_path = "test_download.txt";
        let result = download_content(&client, url, full_file_path).await;
        assert!(result.is_ok());
        assert!(std::path::Path::new(full_file_path).exists());
        remove_file(full_file_path).await.unwrap();
    }

    #[tokio::test]
    async fn test_handle_redirect_and_download() {
        let url = "https://httpbin.org/absolute-redirect/1";
        let file_path = "test_redirect";
        let episode_number = 1;
        let result = handle_redirect_and_download(url, file_path, episode_number).await;
        assert!(result.is_ok());
        let full_file_path = format!("Anime/{}/EP-{:03}.mp4", file_path, episode_number);
        assert!(std::path::Path::new(&full_file_path).exists());
        remove_file(full_file_path).await.unwrap();
        std::fs::remove_dir_all(format!("Anime/{}", file_path)).unwrap();
    }
}
