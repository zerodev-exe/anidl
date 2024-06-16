use colored::Colorize;
use std::fs::File;

pub async fn handle_redirect_and_download(
    url: &str,
    file_path: &str,
    episode_number: u32,
) -> Result<(), reqwest::Error> {
    let downloading_string = format!("{}{}", "Downloading episode ", episode_number);
    let downloaded_episode = format!("{}{}", "Successfully downloaded episode ", episode_number);

    println!("{}", downloading_string.blue());
    let client = reqwest::Client::new();
    let mut current_url = url.to_string();

    let anime_episode = format!("EP-{:03}.mp4", episode_number);
    let full_file_path = format!("Anime/{}/{}", file_path, anime_episode);
    let full_path = format!("Anime/{}/", file_path);

    std::fs::create_dir_all(full_path).unwrap();
    File::create(full_file_path.clone()).unwrap();

    loop {
        let response = client.get(&current_url).send().await?;

        if response.status() == reqwest::StatusCode::FOUND {
            if let Some(location) = response.headers().get(reqwest::header::LOCATION) {
                current_url = location.to_str().unwrap().to_string();
                continue;
            }
        }

        let content = response.bytes().await?;
        tokio::fs::write(full_file_path, content).await.unwrap();
        println!("{}", downloaded_episode.green());
        return Ok(());
    }
}
