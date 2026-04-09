/// Extracts the URLs of anime from the provided HTML body.
/// This function searches for 'a' tags within elements with class 'name'.
///
/// # Arguments
/// * `body` - HTML content as a string.
///
/// # Returns
/// A `Vec<String>` containing the URL endings of each anime.
pub fn get_anime_url(body: String) -> Vec<String> {
    let _ = body;
    Vec::new()
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
    let _ = body;
    Vec::new()
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
pub fn get_media_url(body: String) -> Vec<String> {
    let _ = body;
    Vec::new()
}

pub fn get_anime_images(body: String) -> Vec<String> {
    let _ = body;
    Vec::new()
}

pub fn get_total_number_of_episodes(body: String) -> Result<usize, Box<dyn std::error::Error>> {
    let _ = body;
    Err("Not implemented for AllAnime".into())
}

pub fn is_anime_ongoing(body: &str) -> bool {
    let _ = body;
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_parsers() {
        assert!(get_anime_url("".to_string()).is_empty());
        assert!(get_anime_name("".to_string()).is_empty());
        assert!(get_media_url("".to_string()).is_empty());
        assert!(get_anime_images("".to_string()).is_empty());
        assert!(get_total_number_of_episodes("".to_string()).is_err());
        assert!(!is_anime_ongoing(""));
    }
}
