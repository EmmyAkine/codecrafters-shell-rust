use std::cell::RefCell;
use std::collections::HashMap;
use std::io::Write;
use std::rc::Rc;
use crate::builtins::Command;

pub struct CompleteCommand{
    name : String,
    complete_dict: Rc<RefCell<HashMap<String, String>>>,
}

impl CompleteCommand {
    pub fn new(name : String, complete_dict: &Rc<RefCell<HashMap<String, String>>>) -> Self {
        CompleteCommand{
            name,
            complete_dict: Rc::clone(complete_dict)
        }
    }
    fn print_completion(&self, args:&[&str], stdout: &mut dyn Write, stderr: &mut dyn Write) -> bool {
        if args.len() < 2 {
            writeln!(stderr, "{}", "complete: -p: missing argument".to_string()).unwrap();
            return true;
        }
        match self.complete_dict.borrow().get(args[1]) {
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
        self.complete_dict.borrow_mut().insert(args[2].to_string(), args[1].to_string());
        true
    }

    fn unregister_completion(&self, args:&[&str], stderr: &mut dyn Write) -> bool {
        if args.len() < 2 {
            writeln!(stderr, "{}", "complete: -r: missing argument".to_string()).unwrap();
            return true;
        }
        if self.complete_dict.borrow().contains_key(args[1]) {
            self.complete_dict.borrow_mut().remove(args[1]);
        }
        true
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
            "-C" => self.register_completion(args, stderr),
            "-r" => self.unregister_completion(args, stderr),
            _=> true
        }
    }
}