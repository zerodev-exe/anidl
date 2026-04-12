use futures::future::join_all;
use gogoanime_scraper::{allanime, download};
use std::sync::Arc;
use tokio::sync::Semaphore;
use tokio::task;

use crate::print_handleing::*;

pub async fn get_anime_episodes_and_download_the_episodes(
    anime_url_ending: String,
    path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut tasks = vec![];
    let semaphore = Arc::new(Semaphore::new(4));

    let videos_dir = dirs::video_dir()
        .ok_or("Could not find the Videos directory")
        .unwrap();
    let full_path = videos_dir.join("Anime").join(path);

    let episodes = allanime::episodes_list(&anime_url_ending, "sub").await?;
    if episodes.is_empty() {
        return Err("No episodes found for the selected show".into());
    }

    for (index, episode_str) in episodes.iter().enumerate() {
        let episode_number = (index as u32) + 1;
        let anime_episode = format!("EP-{:04}.mp4", episode_number);
        let file_path = full_path.join(anime_episode);

        if process_existing_file(file_path.to_str().unwrap())? {
            continue;
        }

        let task = create_download_task(
            semaphore.clone(),
            anime_url_ending.clone(),
            episode_str.clone(),
            full_path.to_str().unwrap().to_string(),
            episode_number,
        )
        .await;
        tasks.push(task);
    }

    let results = join_all(tasks).await;
    let mut first_error: Option<Box<dyn std::error::Error + Send + Sync>> = None;
    for result in results {
        match result {
            Ok(Ok(())) => {}
            Ok(Err(e)) => {
                println!("Error downloading episode: {}", e);
                if first_error.is_none() {
                    first_error = Some(e);
                }
            }
            Err(e) => {
                println!("Error downloading episode task: {}", e);
                if first_error.is_none() {
                    first_error = Some(Box::new(e));
                }
            }
        }
    }

    if let Some(err) = first_error {
        return Err(err);
    }

    Ok(())
}

async fn download_episode(
    show_id: String,
    episode_str: String,
    path: String,
    episode_number: u32,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut retry_count = 0;
    let max_retries = 5;

    info_print(&format!(
        "Starting the download on episode {episode_number}"
    ));

    loop {
        let links = match allanime::episode_links(&show_id, "sub", &episode_str).await {
            Ok(links) => links,
            Err(_) => {
                if retry_count >= max_retries {
                    return Err(Box::new(std::io::Error::other("No video URL found after multiple retries")));
                }
                retry_count += 1;
                continue;
            }
        };

        let has_non_m3u8 = links.iter().any(|link| !link.is_m3u8);
        let filtered_links = links
            .into_iter()
            .filter(|link| !has_non_m3u8 || !link.is_m3u8);

        let mut best = None;
        for link in filtered_links {
            if best.is_none() {
                best = Some(link);
                continue;
            }
            if let Some(current) = &best {
                let current_quality = current.quality.unwrap_or(0);
                let new_quality = link.quality.unwrap_or(0);
                if new_quality > current_quality {
                    best = Some(link);
                }
            }
        }

        let best = match best {
            Some(link) => link,
            None => {
                if retry_count >= max_retries {
                    return Err(Box::new(std::io::Error::other("No video URL found after multiple retries")));
                }
                retry_count += 1;
                continue;
            }
        };

        match download::handle_redirect_and_download(
            &best.url,
            &path,
            episode_number,
            best.referer.as_deref(),
        )
        .await
        {
            Ok(_) => {
                success_print(&format!(
                    "Successfully downloaded episode {}",
                    episode_number
                ));
                break;
            }
            Err(_) => {
                if retry_count >= max_retries {
                    return Err(Box::new(std::io::Error::other("Failed to handle redirect after multiple retries")));
                }
                retry_count += 1;
                continue;
            }
        }
    }

    Ok(())
}

fn process_existing_file(full_file_path: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let path_to_file = std::path::Path::new(full_file_path);
    if path_to_file.exists() {
        let metadata = std::fs::metadata(full_file_path)?;
        if metadata.len() > 0 {
            return Ok(true);
        }
    }
    Ok(false)
}

async fn create_download_task(
    semaphore: Arc<Semaphore>,
    show_id: String,
    episode_str: String,
    path: String,
    episode_number: u32,
) -> tokio::task::JoinHandle<Result<(), Box<dyn std::error::Error + Send + Sync>>> {
    let permit = semaphore.clone().acquire_owned().await.unwrap();
    let path_clone = path.clone();
    let show_id_clone = show_id.clone();
    let episode_clone = episode_str.clone();
    task::spawn(async move {
        let _permit = permit; // This ensures the semaphore is released when the task completes
        download_episode(show_id_clone, episode_clone, path_clone, episode_number).await
    })
}
