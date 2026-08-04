use std::collections::HashSet;
use crate::builtins::Command;

pub struct TypeCommand{
    name: String,
    builtins: HashSet<String>
}

impl TypeCommand{
    pub fn new(name: String, builtins: HashSet<String>) -> TypeCommand {
        TypeCommand{
            name,
            builtins
        }
    }
}

impl Command for TypeCommand  {
    fn get_name_copy(&self) -> String {
        self.name.clone()
    }
    fn execute(&self, args: &[&str]) -> bool {
        if args.is_empty(){
            println!("type: missing argument");
            return true
        }
        let target = args[0];
        if self.builtins.contains(target){
            println!("{} is a shell builtin", target);
            return true
        }
        println!("{}: not found", target);
        true
    }
}