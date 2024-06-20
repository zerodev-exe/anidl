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

    let document = Document::from(body.as_str());
    for node in document.find(Attr("class", "name").descendant(Name("a"))) {
        let temp_val = node.attr("href").ok_or("href attribute missing")?;
        let anime_url_ending = temp_val.split("/").last().ok_or("No URL ending found")?;
        anime_list.push(anime_url_ending.to_string())
    }

    anime_list.sort();
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

    anime_list.sort();
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::http;
    use tokio;

    /// Tests the `get_anime_url` function.
    #[tokio::test]
    async fn test_get_anime_url() {
        let body =
            http::get_html("https://anitaku.so/search.html?keyword=Kenka%20Dokugaku".to_string())
                .await
                .unwrap();

        let urls = get_anime_url(body).unwrap();
        assert_eq!(
            urls,
            vec![
                "kenka-dokugaku".to_string(),
                "kenka-dokugaku-dub".to_string()
            ]
        );
    }

    /// Tests the `get_anime_name` function.
    #[tokio::test]
    async fn test_get_anime_name() {
        let body =
            http::get_html("https://anitaku.so/search.html?keyword=kenka%20dokugaku".to_string())
                .await
                .unwrap();

        let names = get_anime_name(body);
        println!("{:?}", names);
        assert_eq!(names, vec!["Kenka Dokugaku", "Kenka Dokugaku (Dub)"]);
    }

    #[tokio::test]
    async fn test_get_anime_url_with_empty_body() {
        let body = "".to_string();
        let urls = get_anime_url(body).unwrap();
        assert!(urls.is_empty());
    }
}
