pub mod commands;
pub mod sysinfo;

#[derive(Default)]
pub struct Shell {
    pub input_line: String,
    pub history: Vec<String>,
}

impl Shell {
    pub fn submit(&mut self) {
        let cmd = self.input_line.clone();
        self.history.push(format!("> {}", cmd));
        commands::handle_command(&cmd, &mut self.history);
        self.input_line.clear();
    }
}