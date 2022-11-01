use crate::terminal::commands::Command;
use crate::terminal::filesystem::FileSystem;
#[derive(Debug)]
pub struct ABOUT {
    name: String,
    help_message: String,
}

impl ABOUT {
    pub fn new(name: String, help_message: String) -> ABOUT {
        ABOUT { name, help_message }
    }
}

impl Command for ABOUT {
    fn execute(&self, _args: Vec<String>, _file_system: &mut FileSystem) -> String {
        "".to_string()
    }

    fn get_help_string(&self) -> String {
        format!("\t{:<20}  {}", self.name, self.help_message)
    }
}
