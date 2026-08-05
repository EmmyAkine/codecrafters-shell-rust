use std::fs::OpenOptions;
use std::process::{Command, Stdio};
use crate::path_resolver::PathResolver;
use crate::redirect::RedirectSpec;

pub struct ExternalCommand{
    #[allow(dead_code)]
    resolver: PathResolver,
}

impl ExternalCommand{
    pub fn new(resolver: PathResolver) -> Self {
        ExternalCommand{resolver}
    }

    pub fn try_run(&self, command: &str, args: &[&str], redirect: &RedirectSpec) -> bool {
        let command = command.to_owned();
        // existence check only — NOT used for execution, so argv[0] stays as typed
        if self.resolver.resolve(&command).is_none() {
            return false;
        }

        let stdout = Self::stdio_for(&redirect.stdout_file, redirect.stdout_append);
        let stderr = Self::stdio_for(&redirect.stderr_file, redirect.stderr_append);

        Command::new(command)
            .args(args)
            .stdout(stdout)
            .stderr(stderr)
            .status()
            .expect("failed to execute process");

        true
    }

    fn stdio_for(path: &Option<String>, append: bool) -> Stdio {
        match path {
            Some(p) => {
                let file = OpenOptions::new()
                    .write(true)
                    .create(true)
                    .append(append)
                    .truncate(!append)
                    .open(p)
                    .expect("failed to open redirect target");
                Stdio::from(file)
            }
            None => Stdio::inherit(),
        }
    }
}