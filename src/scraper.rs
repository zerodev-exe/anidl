use colored::*;
use select::document::Document;
use select::predicate::{Attr, Name, Predicate};

/// Extracts the URLs of anime from the provided HTML body.
/// This function searches for 'a' tags within elements with class 'name'.
///
/// # Arguments
/// * `body` - HTML content as a string.
///
/// # Returns
/// A `Vec<String>` containing the URL endings of each anime.
pub fn get_anime_url(body: String) -> Result<Vec<String>, &'static str> {
    let mut anime_list: Vec<_> = vec![];
    let mut number_list = 0;

    let document = Document::from(body.as_str());
    for node in document.find(Attr("class", "name").descendant(Name("a"))) {
        number_list += 1;
        let num = format!("[{}]", number_list);
        println!("{} - {}", num.red(), node.text());
        let temp_val = node.attr("href").ok_or("href attribute missing")?;
        let anime_url_ending = temp_val.split("/").last().ok_or("No URL ending found")?;
        anime_list.push(anime_url_ending.to_string())
    }

    Ok(anime_list)
}

/// Extracts the names of anime from the provided HTML body.
///
/// # Arguments
///
/// * `body` - A string containing the HTML content.
///
/// # Returns
///
/// A vector of strings, each representing the name of an anime.
pub fn get_anime_name(body: String) -> Vec<String> {
    let mut anime_list: Vec<_> = vec![];

    let document = Document::from(body.as_str());
    for node in document.find(Attr("class", "name").descendant(Name("a"))) {
        anime_list.push(node.text());
    }

    anime_list
}

/// Extracts the video URLs from the provided HTML body.
///
/// # Arguments
///
/// * `body` - A string containing the HTML content.
///
/// # Returns
///
/// A vector of strings, each representing a video URL.
pub fn get_video_url(body: String) -> Vec<String> {
    let mut anime_list: Vec<_> = vec![];

    let document = Document::from(body.as_str());
    for node in document.find(Attr("class", "cf-download").descendant(Name("a"))) {
        anime_list.push(node.attr("href").unwrap().to_string());
    }

    anime_list
}

// TODO: Actually use the URLs in the test cases
#[cfg(test)]
mod tests {
    use crate::http;

    use super::*;
    use tokio;

    /// Tests the `get_anime_url` function.
    #[tokio::test]
    async fn test_get_anime_url() {
        let body = http::get_html("https://animepahe.com/anime/one-piece".to_string())
            .await
            .unwrap();

        let urls = get_anime_url(body).unwrap();
        assert_eq!(urls, vec!["one-piece".to_string()]);
    }

    /// Tests the `get_anime_name` function.
    #[tokio::test]
    async fn test_get_anime_name() {
        let body = r#"<div class="name"><a>One Piece</a></div>"#.to_string();

        let names = get_anime_name(body);
        assert_eq!(names, vec!["One Piece"]);
    }

    /// Tests the `get_video_url` function.
    #[tokio::test]
    async fn test_get_video_url() {
        let body =
            r#"<div class="cf-download"><a href="http://example.com/video.mp4">Download</a></div>"#
                .to_string();

        let urls = get_video_url(body);
        assert_eq!(urls, vec!["http://example.com/video.mp4"]);
    }

    #[tokio::test]
    async fn test_get_anime_url_with_empty_body() {
        let body = "".to_string();
        let urls = get_anime_url(body).unwrap();
        assert!(urls.is_empty());
    }
}

