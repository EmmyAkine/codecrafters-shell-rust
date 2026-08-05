#[allow(unused_imports)]
use std::collections::{HashMap, HashSet};
use crate::builtins::Command;
use crate::external_command::ExternalCommand;

pub struct CommandDispatcher{
    commands: HashMap<String, Box<dyn Command>>,
    #[allow(dead_code)]
    external_command: ExternalCommand
}

impl CommandDispatcher {
    pub fn new(commands: HashMap<String, Box<dyn Command>>, external_command: ExternalCommand) -> CommandDispatcher {
        let dispatcher = CommandDispatcher{commands, external_command };
        dispatcher
    }

    pub fn dispatch(&self, input: &str) -> bool {
        let parts: Vec<&str> = input.trim().split(' ').collect();
        if parts.is_empty() {
            println!("Invalid input!");
            return true;
        }
        let command = parts[0];
        let args = &parts[1..];

        match self.commands.get(command) {
            Some(command) => command.execute(args),
            None => {
                if self.external_command.try_run(&command.to_string(), args) {
                    return true;
                }
                println!("{}: command not found", command.trim());
                true
            }
        }
    }

}