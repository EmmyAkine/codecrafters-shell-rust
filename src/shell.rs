use std::collections::HashMap;
#[allow(unused_imports)]
use std::io;
use std::io::Write;
use crate::builtins::Command;
use crate::builtins::exit::ExitCommand;
use crate::command_dispatcher::CommandDispatcher;
use crate::external_command::ExternalCommand;

pub struct Shell{}

impl Shell {
    pub fn run(&self) {
        let external_commands = ExternalCommand{};
        let builtins = Self::build_builtins();
        let dispatcher = CommandDispatcher::new(builtins, external_commands);
        Self::run_loop(dispatcher);
    }

    fn build_builtins() -> HashMap<String, Box<dyn Command>> {
        let mut builtins: HashMap<String, Box<dyn Command>> = HashMap::new();
        let exit_cmd = ExitCommand{name: "exit".to_string()};
        builtins.insert("exit".to_string(), Box::new(exit_cmd));
        
        builtins
    }

    fn run_loop(dispatcher: CommandDispatcher){
        loop {
            print!("$ ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let dis = dispatcher.dispatch(&input);
            if !dis { break }
        }

    }
}
