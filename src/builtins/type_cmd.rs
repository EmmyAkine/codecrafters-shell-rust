use std::collections::HashSet;
use crate::builtins::Command;
use crate::path_resolver::PathResolver;

pub struct TypeCommand{
    name: String,
    builtins: HashSet<String>,
    resolver: PathResolver,
}

impl TypeCommand{
    pub fn new(name: String, builtins: HashSet<String>, resolver: PathResolver) -> Self {
        TypeCommand{
            name,
            builtins,
            resolver
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
        let target = args[0].to_string();
        if self.builtins.contains(&target){
            println!("{} is a shell builtin", target);
            return true
        }

        if let Some(full_path) = self.resolver.resolve(&target){
            println!("{} is {}", target, full_path);
        }
        else {
            println!("{}: not found", target);
        }
        true
    }
}