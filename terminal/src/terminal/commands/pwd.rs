use crate::terminal::commands::Command;
use crate::terminal::filesystem::FileSystem;

#[derive(Debug)]
pub struct PWD {
    name: String,
    help_message: String,
}

impl PWD {
    pub fn new(name: String, help_message: String) -> PWD {
        PWD { name, help_message }
    }
}

impl Command for PWD {
    fn execute(&self, _args: Vec<String>, file_system: &mut FileSystem) -> String {
        file_system.current_path().clone()
    }

    fn get_help_string(&self) -> String {
        format!("\t{:<20}  {}", self.name, self.help_message)
    }
}
