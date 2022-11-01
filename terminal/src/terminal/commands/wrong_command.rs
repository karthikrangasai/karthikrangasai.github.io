use crate::terminal::commands::Command;
use crate::terminal::filesystem::FileSystem;

#[derive(Debug)]
pub struct WRONGCMD {
    wrong_command: String,
}

impl WRONGCMD {
    pub fn new(wrong_command: String) -> WRONGCMD {
        WRONGCMD { wrong_command }
    }
}

impl Command for WRONGCMD {
    fn execute(&self, _args: Vec<String>, _file_system: &mut FileSystem) -> String {
        format!("`{}` is an invalid command.", self.wrong_command)
    }

    fn get_help_string(&self) -> String {
        "".to_string()
    }
}
