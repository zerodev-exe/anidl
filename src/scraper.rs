pub fn get_anime_episodes(url: String) {
    println!("{}", url);
}

// =======================================================================
// NOTE: BEGINING OF THE FILE DOWNLOAD PART
// =======================================================================

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Specify a vector of URLs of the files you want to download
    let urls: Vec<&str> = vec![];

    // Iterate through each URL in the vector
    for url in urls {
        use download_rs::sync_download::Download;

        let filename = "download/";
        let download = Download::new(url, Some(filename), None);

        match download.download() {
            Ok(_) => println!("Download has been completed for : {}", url),
            Err(e) => println!("There was an error while downloading : {}", e.to_string()),
        }
    }

    Ok(())
}

// =======================================================================
// NOTE: END OF THE DOWNLOAD PART
// =======================================================================
