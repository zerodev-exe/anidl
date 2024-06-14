use reqwest::Url;
use std::fs::File;
use std::io::{self, Write, copy};
use indicatif::{ProgressBar, ProgressStyle};

pub async fn download_file(url: String, file_path: String) -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get(Url::parse(&url)?).await?;
    let total_size = response.content_length().ok_or("Failed to get content length")?;

    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")?
        .progress_chars("#>-"));

    let mut dest = File::create("Anime/".to_string() + &file_path)?;
    let mut content = response.bytes().await?;

    let mut pos = 0;
    while pos < content.len() {
        let chunk_size = std::cmp::min(8192, content.len() - pos);
        let chunk = &content[pos..pos + chunk_size];
        dest.write_all(chunk)?;
        pb.inc(chunk.len() as u64);
        pos += chunk_size;
    }

    pb.finish_with_message("download completed");
    Ok(())
}
