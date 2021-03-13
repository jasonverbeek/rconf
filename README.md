# rconf

Helper for modifying system configs by mapping a name/key to the config path.  

## Example
```~ > rconf nvim```  
Will open the nvim config file in the configured editor (vim by default).

## Installation
Will soon be on the AUR and maybe other distros.  
For now checkout and `cargo build --release` to build the binary to `target/release/rconf`  
The binary can then be placed in `~/.local/bin/`  
Make sure `~/.local/bin/` in on your `$PATH`  

## Configuration
After running `rconf` once you can find a configuration file at `~/.config/rconf/rconf.toml`  
Currently the config file contains 2 examples (vim, nvim). you can add your own configs here like so:  
```
[[ config ]]
name = 'qtile'
path = '~/.config/qtile/config.py'
default_path = '/dev/null'
```
`name` name used with rconf (`rconf <name>`). the executable name (vim/nvim) is used in the example config.  
`path` path to the config file.  
`default_path` (*unimplemented*) path to an example config to use when the `path` file is missing.
