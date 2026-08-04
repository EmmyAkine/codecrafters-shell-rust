pub mod exit;

pub trait Command {
    fn name(&self) -> &str;
    fn set_name(&mut self, new_name: String);
    fn execute(&self, args: &[&str]) -> bool;
}