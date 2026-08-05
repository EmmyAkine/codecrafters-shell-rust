use std::env;
use std::io::Write;
use crate::builtins::Command;

pub struct PwdCommand {
    name: String
}

impl PwdCommand {
    pub fn new(name: String) -> Self {
        PwdCommand{name}
    }
}

impl Command for PwdCommand {
    fn get_name_copy(&self) -> String {
        self.name.clone()
    }
    fn execute(&self, _args: &[&str], stdout: &mut dyn Write, _stderr: &mut dyn Write) -> bool {
        let current_dir = env::current_dir().unwrap().to_string_lossy().into_owned();
        writeln!(stdout, "{}", current_dir).unwrap();
        true
    }
}