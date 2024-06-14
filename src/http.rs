use std::fs::File;

pub async fn get_html(url: String) -> String {
    let res = reqwest::get(url).await.unwrap();
    let body = res.text().await.unwrap();
    return body;
}

pub async fn handle_redirect_and_download(
    url: &str,
    file_path: &str,
    anime_episode: u32,
) -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();
    let mut current_url = url.to_string();

    let episode_number = format!("EP-{:03}.mp4", anime_episode);
    let full_file_path = format!("Anime/{}/{}", file_path, episode_number);
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
        return Ok(());
    }
}
