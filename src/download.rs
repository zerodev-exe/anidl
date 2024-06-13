use download_rs::async_download::Download;

async fn main(url: &str, path: &str) {
    let anime_path = "Anime/";
    let filename = format!("{}{}", anime_path, path);

    let download = Download::new(url, Some(&filename), None);
    match download.download_async().await {
        Ok(_) => println!("下载完成"),
        Err(e) => println!("下载出错 ： {}", e.to_string()),
    }
}
