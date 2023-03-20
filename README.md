# rechifina

**REplace CHars In FIleNAmes**


* replace a given char from a filename with another given char.
* if the path to a directory is given as the last argument, it will go through every entry of the directory.
* it doesn`t go recursively through the directory
* skips a file if the filename doesn`t contain the given char to replace
* by default the user has to confirm the file operation for every file
* use the ```-a; --all``` flag to only confirm once for all files in a directory
* use "." to take the current directory as the <path> argument

## Usage 

# Short Usage

```
rechifina [OPTIONS] [COMMANDS]

Commands:
  log, -L, --log  Show content of the log file
  help            Print this message or the help of the given subcommand(s)

Options:
  -r, --replace <CHAR_TO_REPLACE> <NEW_CHAR> <PATH>
          Replace a given char with a new one in a given file or directory
  -a, --all
          Rename all files without confirmation
  -h, --help
          Print help (see more with '--help')
  -V, --version
          Print version
```
# Long Usage

```
rechifina [OPTIONS] [COMMAND]

Commands:
  log, -L, --log
          Show content of the log file
  help
          Print this message or the help of the given subcommand(s)

Options:
  -r, --replace <CHAR_TO_REPLACE> <NEW_CHAR> <PATH>
          First argument must be the char to replace
          Second argument must be the new char
          Last argmument must be the path to the file or directory
          Use "." to take the current directory as the <PATH> argument

  -a, --all
          Rename all files without confirmation

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

## Examples

```rechifina --replace " " "_" "this is the testfile.txt"```

* changes the filename "this is the testfile.txt" to "this_is_the_testfile.txt"
* asks the user to confirm before the file gets renamed
* shows the possible new filename before changing the filename


![screenshot](https://github.com/Phydon/rechifina/blob/master/assets/rech_prompt.png)

![screenshot](https://github.com/Phydon/rechifina/blob/master/assets/rech_success.png)


```rechifina -r " " "_" .```

* goes through every file in the current directory
* replace every instance of a space (" ") in a filename with an underscore ("_")
* skips all files if there`s no space in the filename
* skips hidden files, directories, symlinks
* asks the user to confirm the renaming for every single file
* shows the possible new filename before changing the filename


```rechifina --replace " " "_" . --all```

* goes through every file in the current directory
* replace every instance of a space (" ") in a filename with an underscore ("_")
* skips all files if there`s no space in the filename
* skips hidden files, directories, symlinks
* asks the user to confirm once before renaming all files in the directory

## Installation

### Windows

via Cargo or get the ![binary](https://github.com/Phydon/rechifina/releases)
