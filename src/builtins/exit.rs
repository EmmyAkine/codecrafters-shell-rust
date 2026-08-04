use crate::builtins::Command;

pub struct ExitCommand{
    pub name: String
}
impl Command for ExitCommand {
    fn name(&self) -> &str { &self.name }
    fn set_name(&mut self, new_name: String) {
        self.name = new_name;
    }
    fn execute(&self, _args: &[&str]) -> bool {
        false
    }
}