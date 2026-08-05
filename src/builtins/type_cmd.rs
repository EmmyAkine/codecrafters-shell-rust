use std::collections::HashSet;
use std::io::Write;
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
    fn execute(&self, args: &[&str], stdout: &mut dyn Write, stderr: &mut dyn Write) -> bool {
        if args.is_empty(){
            writeln!(stderr, "type: missing argument").unwrap();
            return true
        }
        let target = args[0].to_string();
        if self.builtins.contains(&target){
            writeln!(stdout, "{} is a shell builtin", target).unwrap();
            return true
        }

        if let Some(full_path) = self.resolver.resolve(&target){
            writeln!(stdout, "{} is {}", target, full_path).unwrap();
        }
        else {
            writeln!(stderr, "{}: not found", target).unwrap();
        }
        true
    }
}