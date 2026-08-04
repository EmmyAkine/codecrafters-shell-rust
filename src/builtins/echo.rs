use crate::builtins::Command;

pub struct EchoCommand{
    pub name: String
}

impl Command for EchoCommand  {
    fn name(&self) -> &str{ &self.name }

    fn set_name(&mut self, new_name: String) {
        self.name = new_name;
    }

    fn execute(&self, args: &[&str]) -> bool {
        println!("{}", args.join(" "));
        true
    }
}