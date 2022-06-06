use wasm_bindgen::prelude::*;
extern crate web_sys;

use super::commands::help_output;
use super::commands::no_args_for_command_output;
use super::commands::CommandOutput;
use super::utils::log;

#[wasm_bindgen]
pub struct Terminal {
    user: String,
    machine_name: String,
    current_path: String,
}

impl Terminal {
    fn concatenate_paths(&self, mut root: String, dir: String) -> String {
        root.push_str("/");
        root.push_str(&dir);
        root
    }

    fn home_path(&self) -> String {
        self.concatenate_paths("/home".to_string(), self.user())
    }

    pub fn get_path(&self) -> String {
        self.current_path().replacen("~", self.home_path().as_ref(), 1)
    }

    pub fn new() -> Terminal {
        let user: String = String::from("guest");
        let machine_name: String = String::from("karthikrangasai.github.io");
        let current_path: String = String::from("~");

        return Terminal {
            user,
            machine_name,
            current_path,
        };
    }

    fn parse_command(&self, full_command: String) -> (String, Option<Vec<String>>) {
        let mut tokens: Vec<String> = full_command.split(" ").map(str::to_string).collect();
        let args = tokens.split_off(1);
        let command: String = tokens[0].clone();
        if args.len() == 0 {
            return (command, None);
        }
        (command, Some(args))
    }

    fn execute_command(&self, command: String, command_args: Vec<String>) -> String {
        log!("Running command: {} {:?}", command, command_args);
        match command.as_str() {
            "help" => {
                if command_args.len() != 0 {
                    return no_args_for_command_output(command, command_args);
                }
                return help_output();
            }
            "whoami" => {
                if command_args.len() != 0 {
                    return no_args_for_command_output(command, command_args);
                }
                return self.user();
            }
            "about" => {
                if command_args.len() != 0 {
                    return no_args_for_command_output(command, command_args);
                }
                return "".to_string();
            }
            "pwd" => {
                if command_args.len() != 0 {
                    return no_args_for_command_output(command, command_args);
                }
                return self.get_path();
            }
            "clear" => {
                if command_args.len() != 0 {
                    return no_args_for_command_output(command, command_args);
                }
                return "".to_string();
            }
            _ => log!("Invalid command: {} {:?}", command, command_args),
        };
        format!("Invalid command: `{}`.", command)
    }
}

#[wasm_bindgen]
impl Terminal {
    #[wasm_bindgen(getter)]
    pub fn user(&self) -> String {
        self.user.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_user(&mut self, user: String) {
        self.user = user;
    }

    #[wasm_bindgen(getter)]
    pub fn machine_name(&self) -> String {
        self.machine_name.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_machine_name(&mut self, machine_name: String) {
        self.machine_name = machine_name;
    }

    #[wasm_bindgen(getter)]
    pub fn current_path(&self) -> String {
        self.current_path.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_current_path(&mut self, current_path: String) {
        self.current_path = current_path;
    }

    pub fn run_command(&self, full_command: String) -> JsValue {
        let (command, some_args) = self.parse_command(full_command.clone());
        let args: Vec<String>;

        if some_args.is_none() {
            args = [].to_vec();
        } else {
            args = some_args.unwrap();
        }

        let command_output: String = self.execute_command(command.clone(), args.clone());

        let js_output = CommandOutput {
            user: self.user(),
            path: self.current_path(),
            command: full_command.clone(),
            output: command_output,
        };

        JsValue::from_serde(&js_output).unwrap()
    }
}
