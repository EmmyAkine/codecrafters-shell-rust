use std::io::Read;
use std::process::{Command, Stdio};
use crate::path_resolver::PathResolver;

pub struct ExternalCommand{
    #[allow(dead_code)]
    resolver: PathResolver,
}

impl ExternalCommand{
    pub fn new(resolver: PathResolver) -> Self {
        ExternalCommand{resolver}
    }

    pub fn try_run(&self,command: &String, args: &[&str])-> bool{
        let full_path:String;
        if let Some(path) = self.resolver.resolve(&command){
            full_path = path;
        }
        else {
            return false;
        }
        let mut child = Command::new(&full_path).args(args).stdout(Stdio::piped()).spawn().expect("failed to execute process");
        let mut output = String::new();
        //child.stdout.unwrap().read_to_string(&mut output).expect("failed to read stdout");
        child.stdout.as_mut().unwrap().read_to_string(&mut output).expect("failed to read stdout");
        child.wait().unwrap();
        let output = output.trim();
        println!("{output}");

        true
    }
}