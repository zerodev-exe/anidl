pub fn get_anime_episodes(url: String) {
    println!("{}", url);
}

// =======================================================================
// NOTE: BEGINING OF THE FILE DOWNLOAD PART
// =======================================================================

use reqwest;
use std::fs::File;
use std::io::copy;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Specify a vector of URLs of the files you want to download
    let urls = vec![
        "https://example.com/path/to/file1.txt",
        "https://example.com/path/to/file2.txt",
        // Add more URLs as needed
    ];

    // Iterate through each URL in the vector
    for url in urls {
        // Send an HTTP GET request to the URL
        let response = reqwest::get(url).await?;

        // Check if the request was successful (status code 200 OK)
        if response.status().is_success() {
            // Get the file content as a byte stream
            let content = response.bytes().await?;

            // Extract the file name from the URL and use it to create a local file path
            let file_name = url.split('/').last().unwrap_or("unknown_file");
            let local_file_path = format!("path/to/save/{}", file_name);

            // Create a new file or truncate an existing file
            let mut file = File::create(&local_file_path)?;

            // Write the downloaded content to the local file
            copy(&mut content.as_ref(), &mut file)?;

            println!("File downloaded successfully to: {}", local_file_path);
        } else {
            // Print an error message if the request was not successful
            eprintln!("Error: {} for URL: {}", response.status(), url);
        }
    }

    Ok(())
}

// =======================================================================
// NOTE: END OF THE DOWNLOAD PART
// =======================================================================
