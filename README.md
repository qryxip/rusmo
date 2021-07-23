<div align="center">
<h1>rusmo</h1>
rusmo assist your memo life to improve your productivity.

![demo](https://raw.githubusercontent.com/wiki/hashue/rusmo/images/rusmo-demo.gif)

rusmo is a command which is create,edit,delete markdown file on your favorit editor.
</div>

# Usage
```
Usage:
  rusmo
  rusmo (new    | n) [-t <filename>]
  rusmo (edit   | e) <filename>
  rusmo (list   | l)
  rusmo (config | c)
  rusmo (delete | d) <filename>
  rusmo (--help | --version)

Options:
  -h, --help     Show this screen
  -v, --version  Show version
  -t <filename>  create with title

  ``````

# Requiment
- [fzf](https://github.com/junegunn/fzf)

# Installation
rusmo is available for macOS and Linux.

**If you have a Rust environment set up yet, you have to set up Rust environment.**

You can use the cargo install command:

`$ cargo install rusmo`

# Configuration
run `rusmo config` or `rusmo c`

``````
editor = "vim"          #your favorite editor
path = "/path/to/you/"  #file will save here
extension = "md"        #your favorite file extension

``````
# License
MIT
# Author
Hasu

