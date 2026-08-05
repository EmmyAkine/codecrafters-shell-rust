use crate::builtins::Command;

pub struct ExitCommand{
    name: String
}

impl ExitCommand {
    pub fn new(name: String) -> Self {
        ExitCommand{name}
    }
}
impl Command for ExitCommand {
    fn get_name_copy(&self) -> String { self.name.clone() }
    fn execute(&self, _args: &[&str]) -> bool {
        false
    }
}