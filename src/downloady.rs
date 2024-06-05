pub async fn download(urls: Vec<&str>) -> Result<(), Box<dyn std::error::Error>> {
    // Iterate through each URL in the vector
    for url in urls {
        use download_rs::sync_download::Download;

        let filename = "Anime/";
        let download = Download::new(url, Some(filename), None);

        match download.download() {
            Ok(_) => println!("Download has been completed for : {}", url),
            Err(e) => println!("There was an error while downloading : {}", e.to_string()),
        }
    }

    Ok(())
}
