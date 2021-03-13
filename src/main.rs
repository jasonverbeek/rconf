use std::env;

const CONFIG_FILES: &'static [&'static [&'static str]] = &[
    &["qtile", "~/.config/qtile/config.py", "/dev/null"],
    &["nvim", "~/.config/nvim/init.vim", "/dev/null"],
    &["alacritty", "~/.config/alacritty/customize.yml", "/dev/null"]
];

fn find_config(program: &str) {
    println!("{:#?}", CONFIG_FILES[0][0] == program);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    find_config("qtile")
}
