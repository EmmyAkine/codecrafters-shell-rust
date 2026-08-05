use std::env::{ set_current_dir};
use std::{env, path};
use std::io::Write;
use crate::builtins::Command;

pub struct CdCommand {
    name: String,
}

impl CdCommand {
    pub fn new(name: String) -> Self {
        CdCommand{name: name.to_string()}
    }
}

impl Command for CdCommand {
    fn get_name_copy(&self) -> String {
        self.name.clone()
    }

    fn execute(&self, args: &[&str], _stdout: &mut dyn Write, stderr: &mut dyn Write) -> bool {
        let path = args.join(" ");
        if path == "~" {
            _= set_current_dir(env::home_dir().unwrap().to_string_lossy().into_owned());
            return true;
        }
        if path::Path::new(&path).is_dir() {
            _= set_current_dir(path);
        }
        else {
            writeln!(stderr, "cd: {}: No such file or directory", path).unwrap();
        }
        true
    }
}