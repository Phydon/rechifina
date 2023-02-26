# rechifina

**REplace CHars In FIleNAmes**


* replace a given char from a filename with another given char.
* if the path to a directory is given as the last argument, it will go through every entry of the directory.
* skips a file if the filename doesn`t contain the given char to replace
* by default the user has to confirm the file operation for every file
* use the ```-a; --all``` flag to only confirm once for all files in a directory
* use "." to take the current directory as the <path> argument

## Usage 

```
rechifina [OPTIONS] [COMMANDS]

Commands:
  log, -l  Show content of the log file
  help     Print this message or the help of the given subcommand(s)

Options:
  -r, --replace <CHAR_TO_REPLACE> <NEW_CHAR> <PATH>
          First argument must be the char to replace,
          second argument must be the new char,
          last argmument must be the path to the file or directory
  -a, --all
          Rename all files without confirmation
  -h, --help
          Print help (see more with '--help')
  -V, --version
          Print version
```

## Examples

```rechifina --replace " " "_" "test file.txt"```

>changes the filename "test file.txt" to "test_file.txt"

> asks the user to confirm before the file gets renamed

> shows the possible new filename before changing the filename


```rechifina -r " " "_" .```

> goes through every file in the current directory

> replace every instance of a space (" ") in a filename with an underscore ("_")

> skips all files if there`s no space in the filename

> skips hidden files, directories, symlinks

> asks the user to confirm the renaming for every single file

> shows the possible new filename before changing the filename



```rechifina --replace " " "_" . --all```

> goes through every file in the current directory

> replace every instance of a space (" ") in a filename with an underscore ("_")

> skips all files if there`s no space in the filename

> skips hidden files, directories, symlinks

> asks the user to confirm once before renaming all files in the directory


## Installation

* build via cargo
