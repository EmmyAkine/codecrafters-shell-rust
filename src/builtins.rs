pub mod exit;
pub mod echo;
pub mod type_cmd;
pub mod pwd;

pub trait Command {
    fn get_name_copy(&self) -> String;
    fn execute(&self, args: &[&str]) -> bool;
}