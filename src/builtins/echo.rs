use crate::builtins::Command;

pub struct EchoCommand{
    name: String
}

impl EchoCommand{
    pub fn new(name: String) -> EchoCommand {
        EchoCommand{name}
    }
}

impl Command for EchoCommand  {
    fn get_name_copy(&self) -> String{ self.name.clone() }
    fn execute(&self, args: &[&str]) -> bool {
        println!("{}", args.join(" "));
        true
    }
}