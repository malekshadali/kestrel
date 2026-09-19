use super::sysinfo::get_sysinfo;

pub fn handle_command(cmd: &str, history: &mut Vec<String>) {
    match cmd.trim() {
        "clear" => history.clear(),
        cmd if cmd.starts_with("echo ") => {
            let text = &cmd["echo ".len()..];
            history.push(text.to_string());
        }
        "help" => {
            history.push("commands: ".to_string());
            history.push("  clear        - clears the terminal".to_string());
            history.push("  echo <text>  - prints text".to_string());
            history.push("  help         - shows this menu".to_string());
            history.push("  sysinfo      - shows cpu and ram usage".to_string());
        }
        "sysinfo" => {
            for line in get_sysinfo() {
                history.push(line);
            }
        }
        "" => {}
        _ => history.push(format!("unknown command: {}", cmd)),
    }
}