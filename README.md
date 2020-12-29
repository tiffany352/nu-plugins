# NuShell Plugins

A collection of plugins that I've found useful/necessary in my own projects.

## Plugins

- `from nbt` - Decode binary NBT files from Minecraft world saves. Usage: `open --raw level.dat | from nbt`
- `exists?` - Checks if a file exists. Can be used alongside `if` and
  `open` to open a possibly non-existent file without crashing the
  entire pipeline. Usage: `exists? path/to/file.txt` -> bool. In if
  conditions, use `if $(exists? some_path) == $true {} {}`.

## How to use

For each plugin you want to use, run `cargo install --path [path to plugin]`
in the root of this project. Note that NuShell might fail to
load the plugin unless you store it alongside the builtin plugins, which
is possibly a bug. I have to install them to `C:\Program Files\nu\bin` for
the plugins to work.
