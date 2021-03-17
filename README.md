# rconf v2.0.0

Helper for modifying system configs by mapping a name/key to the config path.

Can also use default/example configs when the original config file doesn't exist yet.

## Examples
```~ > rconf nvim```

Will open the nvim config file in the configured editor (vim by default).

And the following will open the rconf config using rconf for ease of use in cross platform

`~ > rconf rconf`

Specifying a config file

`~ > rconf --config /path/to/config.toml rconf`

Only printing the path

`~ > rconf --print rconf`

Show help

`~ > rconf --help`

## Installation

### Archlinux

You can install rconf using your preferred AUR Helper

`~ > paru -S rconf` (latest tag)

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
- `name` name used with rconf (`rconf <name>`). the executable name (vim/nvim) is used in the example config.
- `path` path to the config file.
- `default_path` default_path will be copied when the `path` file is missing. This can be used to point to example configurations.
    - Use `'/dev/null'` or `''` if there is no example available

----

Default location for configs:

GNU/Linux `~/.config/rconf/rconf.toml`

MacOS `~/Library/Application Support/rs.rconf/rconf.toml`

Windows `%AppData%\rconf\config\rconf.toml`

## Change log

### v2.0.0
- move to directories-next instead of waiting on confy 0.5.0
- add cli parsing
- support `--print` to print path instead of edit the file
- support `--config` to use a different config file
- **NOTE:** to upgrade to 2.0.0 from macOS you need to move your config file
    - Old: ~/Library/Preferences/rs.rconf/rconf.toml
    - New: ~/Library/Application Support/rs.rconf/rconf.toml
    - `mv ~/Library/Preferences/rs.rconf ~/Library/Application Support/`

### v1.1.1
- fix for MacOS config path until confy 0.5.0 is out.
- **NOTE:** in version rconf v2.0.0 you will need to move your config on MacOS
    - Old: ~/Library/Preferences/rs.rconf/rconf.toml
    - New: ~/Library/Application Support/rs.rconf/rconf.toml
    - `mv ~/Library/Preferences/rs.rconf ~/Library/Application Support/`

### v1.1.0
- error messages and example config based on OS
- include rconf itself in the example config so you can run `~ > rconf rconf` to edit the config

### v1.0.0
- Basic functionality to map config files
- initial version

