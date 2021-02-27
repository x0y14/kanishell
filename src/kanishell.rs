// use std::env;
// use std::env::VarError;

mod rule;
mod settings;

use std::fs::File;
use std::io::{BufReader, BufRead};
use std::fs;

use serde_json::{Result, Value};
use crate::kanishell::settings::Settings;
use std::iter::Fuse;

#[derive(Debug)]
pub struct KaniShell {
    home_path: String,
    user_setting: Settings,
}

impl KaniShell {
    pub fn new() -> KaniShell {
        let hp = Self::get_home();
        KaniShell {
            home_path: Self::get_home(),
            user_setting: Settings::new(Self::get_home())
        }
    }

    pub fn init(&mut self) {
        self.load_settings();
        self.user_setting.init();
        println!("{:?}", self.user_setting);
    }

    fn get_home() -> String {
        let result = std::env::var("HOME");
        return match result {
            Ok(v) => {
                String::from(v)
            },
            Err(_) => { String::from("") },
        }
    }

    fn load_settings(&mut self) {
        let settings_path = self.home_path.clone()+"/kanishellrc.json";
        // let settings_path = "/Users/x0y14/dev/rust/kanishell/src/kanishell/kanirc.json".to_string();
        self.user_setting = Settings::new(settings_path.clone());
        // self.user_setting.load();
        println!("{:?}", self.user_setting);
    }
}