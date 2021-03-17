use clap::{App, Arg};
use directories_next::ProjectDirs;
use serde::{Deserialize, Serialize};
use shellexpand::tilde;
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
                    path: _get_config_file().unwrap_or(String::from("./rconf.ini")),
                    default_path: String::from("/dev/null"),
                },
            ],
        }
    }
}

fn _get_config_file() -> std::option::Option<String> {
    Some(
        Path::new(
            ProjectDirs::from("rs", "", "rconf")?
                .config_dir()
                .to_str()?,
        )
        .join("rconf.toml")
        .to_str()?
        .to_owned(),
    )
}

fn get_config_path(matches: &clap::ArgMatches) -> std::option::Option<String> {
    if let Some(config_dir) = matches.value_of("config") {
        Some(config_dir.to_owned())
    } else {
        _get_config_file()
    }
}

fn main() -> std::io::Result<()> {
    let matches: clap::ArgMatches = App::new("rconf")
        .version("2.0.0")
        .author("Jason Verbeek <jason@localhost8080.org>")
        .about("CLI utility to quickly access config file")
        .arg(
            Arg::with_name("name")
                .help("name to query the config file")
                .required(true),
        )
        .arg(
            Arg::with_name("print")
                .short("p")
                .long("print")
                .help("Print path to config instead of opening the file"),
        )
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .takes_value(true)
                .help("Use a different config file"),
        )
        .get_matches();

    let print: bool = matches.is_present("print");
    match get_config_path(&matches) {
        Some(config_file) => {
            let cfg: Configuration = confy::load_path(config_file).unwrap();
            let target = matches.value_of("name").unwrap();
            if let Some(result) = cfg.config.iter().find(|x| &x.name == target) {
                if print {
                    println!("{}", &result.path);
                } else {
                    let expanded = tilde(&result.path).into_owned();
                    if !Path::new(&expanded).exists()
                        && (result.default_path != "/dev/null" || result.default_path != "")
                    {
                        match std::fs::copy(&result.default_path, &expanded) {
                            Ok(_) => {}
                            Err(_) => {}
                        }
                    }
                    std::process::Command::new(&cfg.editor)
                        .arg(&expanded)
                        .status()
                        .expect("error");
                }
            }
        }
        None => {
            println!("Error: Could not find a location to store configuration");
        }
    }
    Ok(())
}
