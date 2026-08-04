use std::collections::{HashMap, HashSet};
#[allow(unused_imports)]
use std::io;
use std::io::Write;
use crate::builtins::Command;
use crate::builtins::echo::EchoCommand;
use crate::builtins::exit::ExitCommand;
use crate::builtins::type_cmd::TypeCommand;
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
        let mut builtins_dict: HashMap<String, Box<dyn Command>> = HashMap::new();
        let mut builtins_list: HashSet<String> = HashSet::new();
        
        //exit cmd
        let exit_cmd = ExitCommand::new("exit".to_string());
        builtins_list.insert(exit_cmd.get_name_copy());
        builtins_dict.insert(exit_cmd.get_name_copy(), Box::new(exit_cmd));
        
        //echo cmd
        let echo_cmd = EchoCommand::new("echo".to_string());
        builtins_list.insert(echo_cmd.get_name_copy());
        builtins_dict.insert(echo_cmd.get_name_copy(), Box::new(echo_cmd));
        
        
        
        
        //type cmd --//MUST BE THE LAST
        builtins_list.insert("type".to_string()); //Manually type out type cmd name
        let type_cmd = TypeCommand::new("type".to_string(), builtins_list);
        builtins_dict.insert(type_cmd.get_name_copy(), Box::new(type_cmd));

        builtins_dict
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
