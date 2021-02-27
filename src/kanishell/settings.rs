use serde_json::{Result, Value};
use std::{fs, io};
use std::ptr::null;
use std::fmt::Error;
// use term::stderr;


#[derive(Debug)]
pub struct Settings {
    pub user_setting_file_path: String,
    user_setting: Value,

    welcome_message: String,
    shortcut_paths: Vec<String>,
    prompt_format: String,
}

impl Default for Settings {
    fn default () -> Settings {
        Settings{
            user_setting_file_path: "".to_string(),
            user_setting: Value::Null,
            welcome_message: "Welcome to Kanishell!(place holder)".to_string(),
            shortcut_paths: Vec::new(),
            prompt_format: "%who_am_i %where_am_i_full $".to_string()
        }
    }
}

impl Settings {
    pub fn new(setting_file_path: String) -> Settings {
        Settings {
            user_setting_file_path: setting_file_path.clone(),
            ..Default::default()
        }
    }

    pub fn init(&mut self) {
        // 設定ファイルを読み込み, user_settingへ代入.
        self.user_setting = self.load_setting_file().unwrap();
        println!("wl: {}", self.welcome_message);
        self.welcome_message = self.get_welcome_message().unwrap();
        println!("wl: {}", self.welcome_message);
        // println!("{:?}", self.user_setting);
        // let _shortcut_paths = self.
    }

    fn load_setting_file(&self) -> Option<Value> {
        let setting = fs::read_to_string(self.user_setting_file_path.clone()).unwrap();
        match serde_json::from_str(&*setting) {
            Ok(j) => Some(j),
            Err(j) => None
        }
    }

    fn get_welcome_message(&self) -> Option<String> {
        let _wl = self.user_setting["welcome-message"].clone();
        if _wl == Value::Null {
            return None;
        }
        return Some(_wl.to_string());
    }

    fn get_shortcut_paths(&self) -> Option<Vec<String>> {
        let shortcut_paths = self.user_setting["path"].clone();
        if shortcut_paths == Value::Null {
            return None;
        }
        let mut paths: Vec<String> = Vec::new();
        for p in shortcut_paths.as_array().unwrap() {
            paths.push(p.as_str().unwrap().to_string());
        }
        return Some(paths);
    }
}