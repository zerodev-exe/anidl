use serde_json::Value;
use std::collections::HashMap;

const ALLANIME_BASE: &str = "allanime.day";
const ALLANIME_API: &str = "https://api.allanime.day";
const ALLANIME_REFR: &str = "https://allmanga.to";
const USER_AGENT: &str =
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:109.0) Gecko/20100101 Firefox/121.0";

#[derive(Clone, Debug)]
pub struct AnimeResult {
    pub id: String,
    pub name: String,
    pub episodes: Option<i64>,
}

#[derive(Clone, Debug)]
pub struct EpisodeLink {
    pub quality: Option<u32>,
    pub url: String,
    pub referer: Option<String>,
    pub is_m3u8: bool,
}

pub async fn search_anime(
    query: &str,
    mode: &str,
) -> Result<Vec<AnimeResult>, Box<dyn std::error::Error>> {
    let gql = r#"query( $search: SearchInput $limit: Int $page: Int $translationType: VaildTranslationTypeEnumType $countryOrigin: VaildCountryOriginEnumType ) { shows( search: $search limit: $limit page: $page translationType: $translationType countryOrigin: $countryOrigin ) { edges { _id name availableEpisodes __typename } }}"#;

    let variables = serde_json::json!({
        "search": {"allowAdult": false, "allowUnknown": false, "query": query},
        "limit": 40,
        "page": 1,
        "translationType": mode,
        "countryOrigin": "ALL",
    });

    let json = graphql_post(gql, variables).await?;
    let edges = json
        .get("data")
        .and_then(|v| v.get("shows"))
        .and_then(|v| v.get("edges"))
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();

    let mut results = Vec::new();
    for edge in edges {
        let id = edge
            .get("_id")
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_string();
        let name = edge
            .get("name")
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_string();
        let episodes = edge
            .get("availableEpisodes")
            .and_then(|v| v.get(mode))
            .and_then(|v| v.as_i64());

        if !id.is_empty() && !name.is_empty() {
            results.push(AnimeResult { id, name, episodes });
        }
    }

    Ok(results)
}

pub async fn episodes_list(
    show_id: &str,
    mode: &str,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let gql = r#"query ($showId: String!) { show( _id: $showId ) { _id availableEpisodesDetail }}"#;
    let variables = serde_json::json!({"showId": show_id});
    let json = graphql_post(gql, variables).await?;

    let list = json
        .get("data")
        .and_then(|v| v.get("show"))
        .and_then(|v| v.get("availableEpisodesDetail"))
        .and_then(|v| v.get(mode))
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();

    let mut episodes = Vec::new();
    for item in list {
        match item {
            Value::String(s) => episodes.push(s),
            Value::Number(n) => episodes.push(n.to_string()),
            _ => {}
        }
    }

    episodes.sort_by(|a, b| {
        let left = parse_episode_number(a);
        let right = parse_episode_number(b);
        left.partial_cmp(&right)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    Ok(episodes)
}

pub async fn episode_links(
    show_id: &str,
    mode: &str,
    episode: &str,
) -> Result<Vec<EpisodeLink>, Box<dyn std::error::Error>> {
    let gql = r#"query ($showId: String!, $translationType: VaildTranslationTypeEnumType!, $episodeString: String!) { episode( showId: $showId translationType: $translationType episodeString: $episodeString ) { episodeString sourceUrls }}"#;
    let variables = serde_json::json!({
        "showId": show_id,
        "translationType": mode,
        "episodeString": episode,
    });
    let json = graphql_post(gql, variables).await?;

    let sources = json
        .get("data")
        .and_then(|v| v.get("episode"))
        .and_then(|v| v.get("sourceUrls"))
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();

    let mut decoded_sources = Vec::new();
    for src in sources {
        let source_name = src
            .get("sourceName")
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_string();
        let source_url = src
            .get("sourceUrl")
            .and_then(|v| v.as_str())
            .unwrap_or_default();
        let trimmed = source_url.trim_start_matches("--");
        let decoded = decode_provider_id(trimmed);
        if !decoded.is_empty() {
            decoded_sources.push((source_name, decoded));
        }
    }

    if decoded_sources.is_empty() {
        return Err("No provider links found for the selected episode".into());
    }

    let provider_order = ["Default", "Yt-mp4", "S-mp4", "Luf-Mp4"];
    let mut provider_ids = Vec::new();
    for provider in provider_order.iter() {
        if let Some(provider_id) = decoded_sources
            .iter()
            .find(|(name, _)| name == provider)
            .map(|(_, id)| id.to_string())
        {
            provider_ids.push(provider_id);
        }
    }
    if provider_ids.is_empty() {
        provider_ids = decoded_sources
            .iter()
            .map(|(_, id)| id.to_string())
            .collect();
    }

    let mut links = Vec::new();
    for provider_id in provider_ids {
        if let Ok(mut provider_links) = fetch_provider_links(&provider_id).await {
            links.append(&mut provider_links);
        }
    }

    if links.is_empty() {
        return Err("No playable links returned".into());
    }

    Ok(links)
}

async fn graphql_post(
    query: &str,
    variables: serde_json::Value,
) -> Result<Value, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let response = client
        .post(format!("{ALLANIME_API}/api"))
        .header("Content-Type", "application/json")
        .header("Referer", ALLANIME_REFR)
        .header("User-Agent", USER_AGENT)
        .json(&serde_json::json!({"variables": variables, "query": query}))
        .send()
        .await?;

    let json = response.json::<Value>().await?;
    Ok(json)
}

async fn fetch_provider_links(
    provider_id: &str,
) -> Result<Vec<EpisodeLink>, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let url = format!("https://{ALLANIME_BASE}{provider_id}");
    let response = client
        .get(&url)
        .header("Referer", ALLANIME_REFR)
        .header("User-Agent", USER_AGENT)
        .send()
        .await?;
    let text = response.text().await?;
    let json = serde_json::from_str::<Value>(&text).unwrap_or(Value::String(text));

    let mut links = Vec::new();
    let mut m3u8_url = None;
    let mut m3u8_referer = None;

    collect_links(&json, &mut links, &mut m3u8_url, &mut m3u8_referer);

    if let Some(url) = m3u8_url {
        links.push(EpisodeLink {
            quality: None,
            url,
            referer: Some(m3u8_referer.unwrap_or_else(|| ALLANIME_REFR.to_string())),
            is_m3u8: true,
        });
    }

    Ok(links)
}

fn collect_links(
    value: &Value,
    links: &mut Vec<EpisodeLink>,
    m3u8_url: &mut Option<String>,
    m3u8_referer: &mut Option<String>,
) {
    match value {
        Value::Object(map) => {
            if let (Some(link), Some(res)) = (map.get("link"), map.get("resolutionStr")) {
                if let (Some(link), Some(res)) = (link.as_str(), res.as_str()) {
                    let quality = res
                        .chars()
                        .filter(|c| c.is_ascii_digit())
                        .collect::<String>()
                        .parse::<u32>()
                        .ok();
                    links.push(EpisodeLink {
                        quality,
                        url: link.to_string(),
                        referer: None,
                        is_m3u8: link.contains(".m3u8"),
                    });
                }
            }

            if let Some(referer) = map.get("Referer").and_then(|v| v.as_str()) {
                *m3u8_referer = Some(referer.to_string());
            }

            if let Some(url) = map.get("url").and_then(|v| v.as_str()) {
                if url.contains("master.m3u8") {
                    *m3u8_url = Some(url.to_string());
                }
            }

            for value in map.values() {
                collect_links(value, links, m3u8_url, m3u8_referer);
            }
        }
        Value::Array(items) => {
            for item in items {
                collect_links(item, links, m3u8_url, m3u8_referer);
            }
        }
        Value::String(s) => {
            if s.contains("master.m3u8") {
                *m3u8_url = Some(s.to_string());
            }
        }
        _ => {}
    }
}

fn decode_provider_id(encoded: &str) -> String {
    let mapping = hex_mapping();
    let mut decoded = String::new();
    let mut i = 0;
    let bytes = encoded.as_bytes();
    while i + 1 < bytes.len() {
        let pair = std::str::from_utf8(&bytes[i..i + 2]).unwrap_or("");
        if let Some(ch) = mapping.get(pair) {
            decoded.push_str(ch);
        }
        i += 2;
    }

    decoded.replace("/clock", "/clock.json")
}

fn parse_episode_number(value: &str) -> f64 {
    value.parse::<f64>().unwrap_or(0.0)
}

fn hex_mapping() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();
    map.insert("79", "A");
    map.insert("7a", "B");
    map.insert("7b", "C");
    map.insert("7c", "D");
    map.insert("7d", "E");
    map.insert("7e", "F");
    map.insert("7f", "G");
    map.insert("70", "H");
    map.insert("71", "I");
    map.insert("72", "J");
    map.insert("73", "K");
    map.insert("74", "L");
    map.insert("75", "M");
    map.insert("76", "N");
    map.insert("77", "O");
    map.insert("68", "P");
    map.insert("69", "Q");
    map.insert("6a", "R");
    map.insert("6b", "S");
    map.insert("6c", "T");
    map.insert("6d", "U");
    map.insert("6e", "V");
    map.insert("6f", "W");
    map.insert("60", "X");
    map.insert("61", "Y");
    map.insert("62", "Z");
    map.insert("59", "a");
    map.insert("5a", "b");
    map.insert("5b", "c");
    map.insert("5c", "d");
    map.insert("5d", "e");
    map.insert("5e", "f");
    map.insert("5f", "g");
    map.insert("50", "h");
    map.insert("51", "i");
    map.insert("52", "j");
    map.insert("53", "k");
    map.insert("54", "l");
    map.insert("55", "m");
    map.insert("56", "n");
    map.insert("57", "o");
    map.insert("48", "p");
    map.insert("49", "q");
    map.insert("4a", "r");
    map.insert("4b", "s");
    map.insert("4c", "t");
    map.insert("4d", "u");
    map.insert("4e", "v");
    map.insert("4f", "w");
    map.insert("40", "x");
    map.insert("41", "y");
    map.insert("42", "z");
    map.insert("08", "0");
    map.insert("09", "1");
    map.insert("0a", "2");
    map.insert("0b", "3");
    map.insert("0c", "4");
    map.insert("0d", "5");
    map.insert("0e", "6");
    map.insert("0f", "7");
    map.insert("00", "8");
    map.insert("01", "9");
    map.insert("15", "-");
    map.insert("16", ".");
    map.insert("67", "_");
    map.insert("46", "~");
    map.insert("02", ":");
    map.insert("17", "/");
    map.insert("07", "?");
    map.insert("1b", "#");
    map.insert("63", "[");
    map.insert("65", "]");
    map.insert("78", "@");
    map.insert("19", "!");
    map.insert("1c", "$");
    map.insert("1e", "&");
    map.insert("10", "(");
    map.insert("11", ")");
    map.insert("12", "*");
    map.insert("13", "+");
    map.insert("14", ",");
    map.insert("03", ";");
    map.insert("05", "=");
    map.insert("1d", "%");
    map
}
