# rconf

Helper for modifying system configs by mapping a name/key to the config path.
Can also use default/example configs when the original config file doesn't exist yet.

## Example
```~ > rconf nvim```

Will open the nvim config file in the configured editor (vim by default).

## Installation

### Archlinux

You can install rconf using your preferred AUR Helper

`~ > paru -S rconf`

### Other Distros

I might add support for other distros later, for now use manual installation on other distros

### Manual

To build rconf you need `cargo` installed.

- Clone rconf `git clone git@github.com:jasonverbeek/rconf.git`
- cd to the repo `cd rconf`
- Build using cargo `cargo build --release`
- Move result to `~/.local/bin/` `mkdir -p ~/.local/bin/ && mv target/release/rconf ~/.local/bin/`
- Make sure `~/.local/bin/` is on your `$PATH`

## Configuration
After running `rconf` once you can find a configuration file at `~/.config/rconf/rconf.toml`

Currently the config file contains 2 examples (vim, nvim). you can add your own configs here like so:
```
[[ config ]]
name = 'qtile'
path = '~/.config/qtile/config.py'
default_path = '/usr/share/doc/qtile/default_config.py'
```
`name` name used with rconf (`rconf <name>`). the executable name (vim/nvim) is used in the example config.

`path` path to the config file.

`default_path` default_path will be copied when the `path` file is missing. This can be used to point to example configurations. Use `'/dev/null'` or `''` if there is no example available

