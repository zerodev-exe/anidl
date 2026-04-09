use std::process::Command;

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

fn execute_command(command: &str, args: &[&str]) -> Result<(), std::io::Error> {
    Command::new(command).args(args).spawn()?;
    Ok(())
}

#[cfg(test)]
mod tests {}
