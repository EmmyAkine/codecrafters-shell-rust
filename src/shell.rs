use crate::auto_completion::Completion;
use crate::builtins::cd_cmd::CdCommand;
use crate::builtins::echo::EchoCommand;
use crate::builtins::exit::ExitCommand;
use crate::builtins::pwd::PwdCommand;
use crate::builtins::type_cmd::TypeCommand;
use crate::builtins::Command;
use crate::command_dispatcher::CommandDispatcher;
use crate::external_command::ExternalCommand;
use crate::path_resolver::PathResolver;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::collections::{HashMap, HashSet};

pub struct Shell{}

impl Shell {
    pub fn run(&self) {
        let path_resolver = PathResolver::new_from_environment();
        let external_commands = ExternalCommand::new(path_resolver.clone());
        let executable_cache = path_resolver.get_all_executables();
        let builtins = Self::build_builtins(path_resolver);
        let dispatcher = CommandDispatcher::new(builtins.0, external_commands);
        Self::run_loop(dispatcher, builtins.1, executable_cache);
    }

    fn build_builtins(resolver: PathResolver) -> (HashMap<String, Box<dyn Command>>, HashSet<String>) {
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

        //pwd cmd
        let pwd_cmd = PwdCommand::new("pwd".to_string());
        builtins_list.insert(pwd_cmd.get_name_copy());
        builtins_dict.insert(pwd_cmd.get_name_copy(), Box::new(pwd_cmd));

        //cd cmd
        let cd_cmd = CdCommand::new("cd".to_string());
        builtins_list.insert(cd_cmd.get_name_copy());
        builtins_dict.insert(cd_cmd.get_name_copy(), Box::new(cd_cmd));

        //type cmd --//MUST BE THE LAST
        builtins_list.insert("type".to_string()); //Manually type out type cmd name
        let type_cmd = TypeCommand::new("type".to_string(), builtins_list.clone(), resolver);
        builtins_dict.insert(type_cmd.get_name_copy(), Box::new(type_cmd));

        (builtins_dict, builtins_list.into_iter().collect())
    }

    fn run_loop(dispatcher: CommandDispatcher, builtin_commands: HashSet<String>, path_executables: HashSet<String>) {

        let mut readline = Editor::new().unwrap();
        readline.set_helper(Some(Completion::new(builtin_commands, path_executables)));
        loop {
            let input = readline.readline("$ ");
            match input {
                Ok(input) => {
                    let input = input.trim();
                    if !dispatcher.dispatch(&input){
                        break
                    }
                }
                Err(ReadlineError::Interrupted) => {
                    println!("^C");
                }
                Err(ReadlineError::Eof) => println!("exit"),
                Err(err) => {
                    eprintln!("Error: {:?}", err);
                    break;
                }
            }
        }

    }
}
