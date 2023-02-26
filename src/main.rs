// TODO create costum errors
use clap::{Arg, ArgAction, Command};
use colored::*;
use flexi_logger::{detailed_format, Duplicate, FileSpec, Logger};
use log::{error, warn};

use std::{
    fs, io,
    path::{Path, PathBuf},
    process,
};

fn main() {
    // handle Ctrl+C
    ctrlc::set_handler(move || {
        println!(
            "{} {}",
            "🤬",
            "Received Ctrl-C! => Exit program!".bold().yellow()
        );
        process::exit(0)
    })
    .expect("Error setting Ctrl-C handler");

    // get config dir
    let config_dir = check_create_config_dir().unwrap_or_else(|err| {
        error!("Unable to find or create a config directory: {err}");
        process::exit(1);
    });

    // initialize the logger
    let _logger = Logger::try_with_str("info") // log warn and error
        .unwrap()
        .format_for_files(detailed_format) // use timestamp for every log
        .log_to_file(
            FileSpec::default()
                .directory(&config_dir)
                .suppress_timestamp(),
        ) // change directory for logs, no timestamps in the filename
        .append() // use only one logfile
        .duplicate_to_stderr(Duplicate::Info) // print infos, warnings and errors also to the console
        .start()
        .unwrap();

    // handle arguments
    let matches = rechifina().get_matches();
    let arg_flag = matches.get_flag("all");
    if let Some(args) = matches.get_many::<String>("") {
        let arg_collection = args.map(|s| s.as_str()).collect::<Vec<&str>>();

        if let Err(err) = replace_chars(arg_collection, arg_flag) {
            error!("Error while trying to change the filenames: {}", err);
            process::exit(1);
        }
    } else {
        match matches.subcommand() {
            Some(("log", _)) => {
                if let Ok(logs) = show_log_file(&config_dir) {
                    println!("{}", "Available logs:".bold().yellow());
                    println!("{}", logs);
                } else {
                    error!("Unable to read logs");
                    process::exit(1);
                }
            }
            _ => unreachable!(),
        }
    }
}

fn rechifina() -> Command {
    Command::new("rechifina")
        .bin_name("rechifina")
        .about("Replace a given char from a filename with another given char")
        .long_about(format!(
            "{}\n\n{}\n{}\n{}",
            "Replace a given char from a filename with another given char.",
            "If the path to a directory is given as the last argument,",
            "it will go through every entry of the directory.",
            "It doesn`t go recursively through the directory."
        ))
        .version("1.0.0")
        .author("Leann Phydon <leann.phydon@gmail.com")
        // FIXME aliases not visible -> why not?
        .visible_alias("rech")
        .arg_required_else_help(true)
        .arg(
            Arg::new("")
                .short('r')
                .long("replace")
                .next_line_help(true)
                .long_help(format!(
                    "{},\n{},\n{}",
                    "First argument must be the char to replace",
                    "second argument must be the new char",
                    "last argmument must be the path to the file or directory"
                ))
                .action(ArgAction::Set)
                .num_args(3)
                .value_names(["CHAR_TO_REPLACE", "NEW_CHAR", "PATH"]),
        )
        .arg(
            Arg::new("all")
                .short('a')
                .long("all")
                .help("Rename all files without confirmation")
                .action(ArgAction::SetTrue),
        )
        .subcommand(
            Command::new("log")
                .short_flag('l')
                .about("Show content of the log file"),
        )
}

fn replace_chars(mut args: Vec<&str>, arg_flag: bool) -> io::Result<()> {
    if let Some(p) = args.pop() {
        let path = Path::new(p);
        let replace_char = args[0];
        let new_char = args[1];

        if path.exists() {
            if path.is_file() {
                let new_name = get_new_name(replace_char, new_char, path);
                let new_path = Path::new(&new_name);
                if let Err(err) = rename_file(path, &new_name, new_path, arg_flag) {
                    error!(
                        "Unable to rename {}. Error: {}",
                        path.display().to_string().italic(),
                        err
                    );
                    process::exit(1);
                }
            } else if path.is_dir() {
                let msg = format!(
                    "{} {} {} {} {} {}",
                    "[❓]".dimmed(),
                    "Do you really want to replace the chars in all files in"
                        .red()
                        .bold(),
                    "[".yellow(),
                    path.display().to_string().italic(),
                    "]".yellow(),
                    "? (y/n)".red().bold(),
                );

                if confirm(&msg) {
                    if fs::read_dir(path)?.count() == 0 {
                        warn!(
                            "The given Directory {} is empty",
                            path.display().to_string().italic()
                        );
                    }

                    for entry in fs::read_dir(path)? {
                        let entry = entry?;

                        if entry.path().is_file() {
                            let new_name = get_new_name(replace_char, new_char, &entry.path());
                            let new_path = Path::new(&new_name);
                            if let Err(err) =
                                rename_file(entry.path().as_path(), &new_name, new_path, arg_flag)
                            {
                                error!(
                                    "Unable to rename {}. Error: {}",
                                    path.display().to_string().italic(),
                                    err
                                );
                                process::exit(1);
                            }
                        }
                    }
                } else {
                    println!("{} {}", "[❌]".dimmed(), "Nevermind then".bold().dimmed());
                    process::exit(0);
                }
            } else {
                error!("Path is not a file or directory");
                process::exit(1);
            }
        } else {
            error!("Path doesn`t exist or you don`t have access");
            process::exit(1);
        }
    } else {
        error!(
            "Missing path argument. The path to the file or directory must be the last argument."
        );
        process::exit(1);
    }

    Ok(())
}

fn get_new_name(replace_char: &str, new_char: &str, path: &Path) -> String {
    let mut new_name = String::new();

    let extension = path
        .extension()
        .unwrap_or_else(|| {
            error!("Not a valid file. Missing extension");
            process::exit(1);
        })
        .to_str()
        .unwrap_or_else(|| {
            error!("Not a valid file. Missing extension");
            process::exit(1);
        });

    // FIXME add parent path to new path/new_name
    // otherwise it will move the files to the current dir
    let parent = path
        .parent()
        .unwrap_or_else(|| Path::new(""))
        .to_str()
        .unwrap_or_else(|| {
            error!("Not a valid file. Missing extension");
            process::exit(1);
        });

    match path.file_stem() {
        Some(filestem_as_osstr) => match filestem_as_osstr.to_str() {
            Some(filestem_as_str) => {
                let rpl_name = filestem_as_str.replace(replace_char, new_char);
                if parent.is_empty() {
                    new_name.push_str(&rpl_name);
                    new_name.push_str(".");
                    new_name.push_str(extension);
                } else {
                    new_name.push_str(parent);
                    new_name.push_str("\\");
                    new_name.push_str(&rpl_name);
                    new_name.push_str(".");
                    new_name.push_str(extension);
                }
            }
            None => {
                error!("Non UTF-8 string found. Path must be valid unicode");
                process::exit(1);
            }
        },
        None => {
            error!("No valid &OsStr found");
            process::exit(1);
        }
    }

    new_name
}

fn rename_file(path: &Path, new_name: &str, new_path: &Path, all_flag: bool) -> io::Result<()> {
    if all_flag {
        fs::rename(path, new_path)?;
        println!(
            "{} {} {} {} {} {} {} {} {}",
            "[✔]".dimmed(),
            "Successfully renamed".green().bold(),
            "[".yellow(),
            path.display().to_string().italic(),
            "]".yellow(),
            "to".green().bold(),
            "[".yellow(),
            new_name.italic(),
            "]".yellow()
        )
    } else {
        let msg = format!(
            "{} {} {} {} {} {}\n{} {} {} {} {}",
            "[❓]".dimmed(),
            "Do you really want to replace the chars in".red().bold(),
            "[".yellow(),
            path.display().to_string().italic(),
            "]".yellow(),
            "? (y/n)".red().bold(),
            " ↪  ".dimmed(),
            "The new path would look like this:".red().bold(),
            "[".yellow(),
            new_name.italic().green(),
            "]".yellow(),
        );

        if confirm(&msg) {
            fs::rename(path, new_path)?;
            println!(
                "{} {} {} {} {} {} {} {} {}",
                "[✔]".dimmed(),
                "Successfully renamed".green().bold(),
                "[".yellow(),
                path.display().to_string().italic(),
                "]".yellow(),
                "to".green().bold(),
                "[".yellow(),
                new_name.italic(),
                "]".yellow()
            );
        } else {
            println!("{} {}", "[❌]".dimmed(), "Nevermind then".bold().dimmed());
        }
    }

    Ok(())
}

fn confirm(msg: &str) -> bool {
    loop {
        println!("{}", msg);

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim().to_lowercase().as_str() {
            "yes" | "y" => return true,
            "no" | "n" => return false,
            _ => {}
        }
    }
}

fn check_create_config_dir() -> io::Result<PathBuf> {
    let mut new_dir = PathBuf::new();
    match dirs::config_dir() {
        Some(config_dir) => {
            new_dir.push(config_dir);
            new_dir.push("rechifina");
            if !new_dir.as_path().exists() {
                fs::create_dir(&new_dir)?;
            }
        }
        None => {
            error!("Unable to find config directory");
        }
    }

    Ok(new_dir)
}

fn show_log_file(config_dir: &PathBuf) -> io::Result<String> {
    let log_path = Path::new(&config_dir).join("rechifina.log");
    match log_path.try_exists()? {
        true => {
            return Ok(format!(
                "{} {}\n{}",
                "Log location:".italic().dimmed(),
                &log_path.display(),
                fs::read_to_string(&log_path)?
            ));
        }
        false => {
            return Ok(format!(
                "{} {}",
                "No log file found:".red().bold().to_string(),
                log_path.display()
            ))
        }
    }
}
