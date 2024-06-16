pub async fn get_html(url: String) -> Result<String, reqwest::Error> {
    let res = reqwest::get(url).await?;
    let body = res.text().await?;

    Ok(body)
}
