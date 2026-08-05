use std::env;
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
    fn execute(&self, _args: &[&str]) -> bool {
        let current_dir = env::current_dir().unwrap().to_string_lossy().into_owned();
        println!("{}", current_dir);
        true
    }
}