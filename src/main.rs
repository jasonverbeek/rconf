use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use shellexpand::tilde;
use std::env;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
struct ConfigurableItem {
    pub name: String,
    pub path: String,
    pub default_path: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Configuration {
    pub editor: String,
    pub config: Vec<ConfigurableItem>,
}

fn get_config_path() -> String {
    // rs , , rconf arguments taken from confy source since confy doesn't allow you to get a path var
    if let Some(proj_dirs) = ProjectDirs::from("rs", "", "rconf") {
        if let Some(config_dir) = proj_dirs.config_dir().to_str() {
            config_dir.to_string()
        } else {
            String::from("")
        }
    } else {
        String::from("")
    }
}

impl ::std::default::Default for Configuration {
    fn default() -> Self {
        Self {
            editor: String::from("vim"),
            config: vec![
                ConfigurableItem {
                    name: String::from("nvim"),
                    path: String::from("~/.config/nvim/init.vim"),
                    default_path: String::from("/dev/null"),
                },
                ConfigurableItem {
                    name: String::from("vim"),
                    path: String::from("~/.vimrc"),
                    default_path: String::from("/dev/null"),
                },
                ConfigurableItem {
                    name: String::from("rconf"),
                    path: get_config_path(),
                    default_path: String::from("/dev/null"),
                },
            ],
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: rconf <name>");
        println!("Add more configs in {}", get_config_path());
        std::process::exit(1);
    }
    let target = &args[1];
    let cfg: Configuration = confy::load("rconf").unwrap();
    match cfg.config.iter().find(|x| &x.name == target) {
        Some(result) => {
            let config_file = tilde(&result.path).into_owned();
            if !Path::new(&config_file).exists()
                && (result.default_path != "/dev/null" || result.default_path != "")
            {
                match std::fs::copy(&result.default_path, &config_file) {
                    Ok(_) => {}
                    Err(_) => {}
                }
            }
            std::process::Command::new(&cfg.editor)
                .arg(config_file)
                .status()
                .expect("error");
        }
        None => println!(
            "config not found for '{}' in ~/.config/rconf/rconf.toml",
            target
        ),
    }
}
