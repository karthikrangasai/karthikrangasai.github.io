use wasm_bindgen::prelude::*;
extern crate reqwest;
extern crate web_sys;
use super::filesystem::FileSystem;
use crate::terminal::commands::{common::CommandOutput, CommandRunner};

#[wasm_bindgen]
#[derive(Debug)]
pub struct Terminal {
    user: String,
    file_system: FileSystem,
    machine_name: String,
    current_path: String,
    command_runner: CommandRunner,
}

impl Terminal {
    pub fn new() -> Terminal {
        let user: String = String::from("guest");
        let file_system: FileSystem = FileSystem::new(user.clone());
        let machine_name: String = String::from("karthikrangasai.github.io");
        let current_path: String = file_system.terminal_display_path();

        // let fs_pointer: Rc<FileSystem> = Rc::new(file_system);
        let command_runner: CommandRunner = CommandRunner::new();

        return Terminal {
            user,
            file_system,
            machine_name,
            current_path,
            command_runner,
        };
    }
}

#[wasm_bindgen]
impl Terminal {
    #[wasm_bindgen(getter)]
    pub fn user(&self) -> String {
        self.user.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn machine_name(&self) -> String {
        self.machine_name.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn current_path(&self) -> String {
        self.current_path.clone()
    }

    pub fn run_command(&mut self, full_command: String) -> JsValue {
        let file_system: &mut FileSystem = &mut self.file_system;
        let command_output: String = self.command_runner.execute_command(full_command.clone(), file_system);

        let js_output: CommandOutput =
            CommandOutput::new(self.user(), self.current_path(), full_command.clone(), command_output);

        serde_wasm_bindgen::to_value(&js_output).unwrap()
    }
}
