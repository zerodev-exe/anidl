use std::process::Command;

pub async fn get_html(url: String) -> Result<String, reqwest::Error> {
    let res = reqwest::get(url).await?;
    let body = res.text().await?;

    Ok(body)
}

pub fn clear_terminal_screen() {
    let result = if cfg!(target_os = "windows") {
        execute_command("cmd", &["/c", "cls"])
    } else {
        execute_command("tput", &["reset"])
    };

    if result.is_err() {
        print!("{esc}c", esc = 27 as char);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_html() {
        let url = "https://httpbin.org/get".to_string();
        let result = get_html(url).await;
        assert!(result.is_ok());
    }
}

fn execute_command(command: &str, args: &[&str]) -> Result<(), std::io::Error> {
    Command::new(command).args(args).spawn()?;
    Ok(())
}
