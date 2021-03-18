use shellexpand::tilde;
use std::path::Path;

mod settings;

fn main() -> Result<(), String> {
    let matches: clap::ArgMatches = settings::get_matches();
    let print: bool = matches.is_present("print");
    let config_file: String =
        settings::get_config_path(&matches).ok_or("Could not access app data directory")?;
    let cfg: settings::Configuration =
        confy::load_path(&config_file).expect(&format!("Failed to load {}", &config_file));
    let target = matches
        .value_of("name")
        .ok_or("Argument name not specified")?;
    let application = cfg
        .config
        .iter()
        .find(|x| &x.name == target)
        .ok_or(&format!("Name {} not found in {}", target, &config_file))?;
    if print {
        println!("{}", &application.path);
    } else {
        let expanded = tilde(&application.path).into_owned();
        if !Path::new(&expanded).exists()
            && (application.default_path != "/dev/null" || application.default_path != "")
        {
            match std::fs::copy(&application.default_path, &expanded) {
                Ok(_) => {}
                Err(_) => {}
            }
        }
        std::process::Command::new(&cfg.editor)
            .arg(&expanded)
            .status()
            .expect(&format!("Failed to run editor {}", &cfg.editor));
    }
    Ok(())
}
