use std::collections::HashMap;
use std::io::Write;
use crate::builtins::Command;

pub struct CompleteCommand{
    name : String,
    complete_dict: HashMap<String, String>,
}

impl CompleteCommand {
    pub fn new(name : String, complete_dict: HashMap<String, String>) -> Self {
        CompleteCommand{
            name,
            complete_dict
        }
    }
}

impl Command for CompleteCommand {
    fn get_name_copy(&self) -> String {
        self.name.clone()
    }

    fn execute(&self, args: &[&str], stdout: &mut dyn Write, stderr: &mut dyn Write) -> bool {

        if args.len() <= 0 {
            return true;
        }

        match args[0] {
            "-p" => self.print_completion(args, stdout, stderr),
            "-C" => self.register_completion(args, stdout),
            "-r" => todo!(),
            _=> true
        }
    }
}

impl CompleteCommand {
    fn print_completion(&self, args:&[&str], stdout: &mut dyn Write, stderr: &mut dyn Write) -> bool {
        if args.len() < 2 {
            writeln!(stderr, "{}", "complete: -p: missing argument".to_string()).unwrap();
            return true;
        }
        match self.complete_dict.get(args[1]) {
            Some(value) => {
                writeln!(stdout, "complete -C '{}' {}", value, args[1] ).unwrap();
                true
            }
            None => {
                writeln!(stderr, "complete: {}: no completion specification", args[1]).unwrap();
                true
            }
        }
    }

    fn register_completion(&self, args:&[&str], stderr: &mut dyn Write) -> bool {
        if args.len() < 2 {
            writeln!(stderr, "{}", "complete: -C: missing argument".to_string()).unwrap();
            return true;
        }

        self.complete_dict.insert(args[2].to_string(), args[1].to_string());
        true
    }
}