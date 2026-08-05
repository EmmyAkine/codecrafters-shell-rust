use std::io::Write;
use crate::builtins::Command;

pub struct EchoCommand{
    name: String
}

impl EchoCommand{
    pub fn new(name: String) -> Self {
        EchoCommand{name}
    }
}

impl Command for EchoCommand  {
    fn get_name_copy(&self) -> String{ self.name.clone() }
    fn execute(&self, args: &[&str],stdout: &mut dyn Write, _stderr: &mut dyn Write) -> bool {
        writeln!(stdout, "{}", args.join(" ").trim()).unwrap();
        true
    }
}