use core::panic;
use std::collections::BTreeMap;

use crate::terminal::utils::log;

#[derive(Clone, Debug)]
struct Path {
    _is_file: bool,
    _name: String,
    _prefix: String,
    children: Option<BTreeMap<String, Path>>,
}

impl Path {
    fn get_path(&self) -> String {
        if self._prefix == "".to_string() && self._name == "/".to_string() {
            return "/".to_string();
        }
        if self._prefix == "/".to_string() {
            return [self._prefix.clone(), self._name.clone()].join("");
        }
        [self._prefix.clone(), self._name.clone()].join("/")
    }

    fn _mkpath(&mut self, dir_or_file_name: String, is_file: bool) -> Result<&Path, String> {
        let current_path = self.get_path();
        if self._is_file {
            return Err(format!("`{}` is a file. Cannot create a directory here.", current_path));
        }

        let new_dir_or_file_path_node = Path {
            _is_file: is_file,
            _name: dir_or_file_name.clone(),
            _prefix: current_path.clone(),
            children: if is_file { None } else { Some(BTreeMap::new()) },
        };

        let children = match self.children {
            Some(ref mut children) => {
                if children.contains_key(&dir_or_file_name) {
                    return Err(format!("`{:?}` is already present.", dir_or_file_name));
                }
                children
            }
            None => {
                // let children: BTreeMap<String, Path> = BTreeMap::new();
                // self.children = Some(children);
                // &children
                panic!();
            }
        };

        children.insert(dir_or_file_name.clone(), new_dir_or_file_path_node);
        Ok(children.get(&dir_or_file_name).unwrap())
    }

    fn mkdir(&mut self, dir_name: String) -> Result<&Path, String> {
        self._mkpath(dir_name, false)
    }

    fn mkfile(&mut self, file_name: String) -> Result<&Path, String> {
        self._mkpath(file_name, true)
    }
}

#[derive(Debug)]
pub struct FileSystem {
    user: String,
    current_path_node: Path,
    root_folder_path_node: Path,
}

impl FileSystem {
    pub fn new(user: String) -> FileSystem {
        let mut root_dir_node = Path {
            _is_file: false,
            _name: "/".to_string(),
            _prefix: "".to_string(),
            children: Some(BTreeMap::new()),
        };

        let home_dir_node_result = root_dir_node.mkdir("home".to_string());
        if home_dir_node_result.is_err() {
            let error_msg = home_dir_node_result.err().unwrap();
            panic!("{}", &error_msg);
        }
        let mut home_dir_node = home_dir_node_result.ok().unwrap().to_owned();

        let guest_dir_node_result = home_dir_node.mkdir(user.clone());
        if guest_dir_node_result.is_err() {
            let error_msg = guest_dir_node_result.err().unwrap();
            panic!("{}", &error_msg);
        }

        let guest_dir_node = guest_dir_node_result.ok().unwrap();

        FileSystem {
            user: user.clone(),
            current_path_node: guest_dir_node.to_owned(),
            root_folder_path_node: root_dir_node,
        }
    }

    // pub fn search(&self, dir_walk_path: &mut Vec<String>) {
    // 	while !dir_walk_path.is_empty() {
    // 		match self.node {}
    // 	}
    // }

    pub fn user(&self) -> String {
        self.user.clone()
    }

    pub fn current_path(&self) -> String {
        log!(
            "self.current_path_node.get_path() = {}.",
            self.current_path_node.get_path()
        );

        log!("self.current_path_node._prefix = {}.", self.current_path_node._prefix);

        log!("self.current_path_node._name = {}.", self.current_path_node._name);

        self.current_path_node.get_path()
    }

    pub fn terminal_display_path(&self) -> String {
        self.current_path().replacen(self.home_path().as_str(), "~", 1)
    }

    fn concatenate_paths(&self, mut root: String, dir: String) -> String {
        root.push_str("/");
        root.push_str(&dir);
        root
    }

    fn home_path(&self) -> String {
        let home_path = ["".to_string(), "home".to_string(), self.user.clone()].join("/");
        log!("Home path is {}.", home_path);
        home_path
    }
}
