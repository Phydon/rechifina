// TODO create costum errors
// TODO add arg: show output as table
// TODO add arg: show no output
use clap::{Arg, ArgAction, Command};
use flexi_logger::{detailed_format, Duplicate, FileSpec, Logger};
use log::{error, warn};
use owo_colors::colored::*;

use std::{
    env, fs, io,
    path::{Path, PathBuf},
    process,
};

fn main() {
    // handle Ctrl+C
    ctrlc::set_handler(move || {
        println!(
            "{} {} {} {}",
            "Received Ctrl-C!".bold().red(),
            "🤬",
            "Exit program!".bold().red(),
            "☠",
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
    let all_flag = matches.get_flag("all");
    let uppercase_flag = matches.get_flag("uppercase");
    let lowercase_flag = matches.get_flag("lowercase");

    if let Some(matches) = matches.get_many::<String>("args") {
        let args = matches.map(|s| s.as_str()).collect::<Vec<&str>>();

        if let Err(err) = replace_chars(args, all_flag, uppercase_flag, lowercase_flag) {
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
        .before_help(format!(
            "{}\n{}",
            "RECHIFINA".bold().truecolor(250, 0, 104),
            "Leann Phydon <leann.phydon@gmail.com>".italic().dimmed()
        ))
        .about(format!(
            "{}{}{}{}{}{}{}{}{}{}",
            "RE".truecolor(250, 0, 104),
            "place a given ",
            "CH".truecolor(250, 0, 104),
            "ar ",
            "I".truecolor(250, 0, 104),
            "n a ",
            "FI".truecolor(250, 0, 104),
            "le",
            "NA".truecolor(250, 0, 104),
            "me with another given char"
        ))
        .long_about(format!(
            "{}{}{}{}{}{}{}{}{}{}\n\n{}\n{}\n{}\n{}\n{}",
            "RE".truecolor(250, 0, 104),
            "place a given ",
            "CH".truecolor(250, 0, 104),
            "ar ",
            "I".truecolor(250, 0, 104),
            "n a ",
            "FI".truecolor(250, 0, 104),
            "le",
            "NA".truecolor(250, 0, 104),
            "me with another given char",
            "If the path to a directory is given as the last argument,",
            "it will go through every entry of the directory",
            "It doesn`t go recursively through the directory",
            "Skips a file if the filename doesn`t contain the given char to replace",
            "By default the user has to confirm the file operation for every file",
        ))
        // TODO update version
        .version("1.1.0")
        .author("Leann Phydon <leann.phydon@gmail.com>")
        .arg_required_else_help(true)
        .arg(
            Arg::new("args")
                .help("Replace a given char with a new one in a given file or directory")
                .next_line_help(true)
                .long_help(format!(
                    "{}\n{}\n{}\n{}",
                    "First argmument must be the path to the file or directory",
                    "Use \".\" to take the current directory as the <PATH> argument",
                    "Second argument must be the char to replace",
                    "Last argument must be the new char",
                ))
                .action(ArgAction::Set)
                .num_args(1..=3)
                .value_names(["PATH", "CHAR_TO_REPLACE", "NEW_CHAR"]),
        )
        .arg(
            Arg::new("all")
                .short('a')
                .long("all")
                .help("Rename all files without confirmation")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("lowercase")
                .short('l')
                .long("lowercase")
                .help("Make the filename all lowercase")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("uppercase")
                .short('u')
                .long("uppercase")
                .help("Make the filename all uppercase")
                .action(ArgAction::SetTrue),
        )
        .subcommand(
            Command::new("log")
                .short_flag('L')
                .long_flag("log")
                .about("Show content of the log file"),
        )
}

fn replace_chars(
    args: Vec<&str>,
    all_flag: bool,
    uppercase_flag: bool,
    lowercase_flag: bool,
) -> io::Result<()> {
    if !args.is_empty() {
        let mut path = Path::new(args[0]);

        let current_dir = env::current_dir()?;

        if path == Path::new(".") {
            path = current_dir.as_path();
        }

        if path.exists() {
            if uppercase_flag || lowercase_flag {
                if path.is_file() {
                    let new_name = show_lower_or_upper_name(path, uppercase_flag, lowercase_flag);
                    let new_path = Path::new(&new_name);
                    if path == new_path {
                        println!(
                            "{} {} {}",
                            "[🤨]".dimmed(),
                            "The filename wouldn`t change".purple().bold(),
                            "💥",
                        );
                        return Ok(());
                    } else if let Err(err) =
                        // rename_file(path, new_path, all_flag, replace_char, new_char)
                        // TODO
                        make_lower_or_upper(path, new_path, all_flag)
                    {
                        error!(
                            "Unable to rename {}. Error: {}",
                            path.display().to_string().italic(),
                            err
                        );
                        process::exit(1);
                    }
                } else if path.is_dir() {
                    let msg = if uppercase_flag {
                        format!(
                            "{} {} {}{}{} {}",
                            "[❓]".dimmed(),
                            "Do you really want to change all files in".red().bold(),
                            "[ \'".yellow(),
                            path.display().to_string().italic(),
                            "\' ]".yellow(),
                            "to uppercase? (y/n)".red().bold(),
                        )
                    } else {
                        format!(
                            "{} {} {}{}{} {}",
                            "[❓]".dimmed(),
                            "Do you really want to change all files in".red().bold(),
                            "[ \'".yellow(),
                            path.display().to_string().italic(),
                            "\' ]".yellow(),
                            "to lowercase? (y/n)".red().bold(),
                        )
                    };

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
                                let new_name =
                                    show_lower_or_upper_name(path, uppercase_flag, lowercase_flag);
                                let new_path = Path::new(&new_name);
                                if path == new_path {
                                    println!(
                                        "{} {} {}",
                                        "[🤨]".dimmed(),
                                        "The filename wouldn`t change".purple().bold(),
                                        "💥",
                                    );
                                    return Ok(());
                                } else if let Err(err) =
                                    // rename_file(path, new_path, all_flag, replace_char, new_char)
                                    // TODO
                                    make_lower_or_upper(path, new_path, all_flag)
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
                // DONE
            } else if args.len() == 3 {
                let replace_char = args[1];
                let new_char = args[2];
                if path.is_file() {
                    let new_name = get_new_name(replace_char, new_char, path);

                    if new_name.is_empty() {
                        return Err(io::Error::from(io::ErrorKind::InvalidData));
                    } else {
                        let new_path = Path::new(&new_name);
                        if path == new_path {
                            println!(
                                "{} {} {}",
                                "[🤨]".dimmed(),
                                "The filename wouldn`t change".purple().bold(),
                                "💥",
                            );
                            return Ok(());
                        } else if let Err(err) =
                            rename_file(path, new_path, all_flag, replace_char, new_char)
                        {
                            error!(
                                "Unable to rename {}. Error: {}",
                                path.display().to_string().italic(),
                                err
                            );
                            process::exit(1);
                        }
                    }
                } else if path.is_dir() {
                    let msg = format!(
                        "{} {} {}{}{} {} {}{}{} {} {}{}{} {}",
                        "[❓]".dimmed(),
                        "Do you really want to replace all".red().bold(),
                        "[ \'".yellow(),
                        replace_char,
                        "\' ]".yellow(),
                        "with".red().bold(),
                        "[ \'".yellow(),
                        new_char,
                        "\' ]".yellow(),
                        "in all files in".red().bold(),
                        "[ \'".yellow(),
                        path.display().to_string().italic(),
                        "\' ]".yellow(),
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
                                if new_name.is_empty() {
                                    continue;
                                } else {
                                    let new_path = Path::new(&new_name);
                                    if entry.path() == new_path {
                                        continue;
                                    } else if let Err(err) = rename_file(
                                        entry.path().as_path(),
                                        new_path,
                                        all_flag,
                                        replace_char,
                                        new_char,
                                    ) {
                                        error!(
                                            "Unable to rename {}. Error: {}",
                                            path.display().to_string().italic(),
                                            err
                                        );
                                        process::exit(1);
                                    }
                                }
                            }
                        }
                    } else {
                        println!("{} {}", "[❌]".dimmed(), "Nevermind then".bold().dimmed());
                        process::exit(0);
                    }
                } else {
                    error!(
                            "Missing command or at least one required argument: [CHAR_TO_REPLACE] [NEW_CHAR]"
                        );
                    process::exit(1);
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
            "Missing path argument. The path to the file or directory must be the first argument."
        );
        process::exit(1);
    }

    Ok(())
}

fn get_new_name(replace_char: &str, new_char: &str, path: &Path) -> String {
    let mut new_name = String::new();

    let extension = path
        .extension()
        .unwrap_or_else(|| std::ffi::OsStr::new(""))
        .to_str()
        .unwrap_or_else(|| {
            error!(
                "Extension from file: {} not convertable to str",
                path.display().to_string().italic()
            );
            process::exit(1);
        });

    let parent = path
        .parent()
        .unwrap_or_else(|| Path::new(""))
        .to_str()
        .unwrap_or_else(|| {
            error!(
                "Parent path from file: {} not convertable to str",
                path.display().to_string().italic()
            );
            process::exit(1);
        });

    match path.file_stem() {
        Some(filestem_as_osstr) => match filestem_as_osstr.to_str() {
            Some(filestem_as_str) => {
                let rpl_name = filestem_as_str.replace(replace_char, new_char);

                if !parent.is_empty() {
                    new_name.push_str(parent);
                    new_name.push_str("\\");
                }

                new_name.push_str(&rpl_name);

                if !extension.is_empty() {
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

fn rename_file(
    path: &Path,
    new_path: &Path,
    all_flag: bool,
    replace_char: &str,
    new_char: &str,
) -> io::Result<()> {
    if all_flag {
        fs::rename(path, new_path)?;
        println!(
            "{} {}\n{} {} {} {} {}\n{} {} {} {} {}",
            "[✔] ".dimmed(),
            "Successfully renamed".green().bold(),
            " ↪  ".dimmed(),
            "Old name:".dimmed(),
            "[".yellow(),
            path.file_name()
                .unwrap()
                .to_string_lossy()
                .italic()
                .dimmed(),
            "]".yellow(),
            " ↪  ".dimmed(),
            "New name:".bright_green(),
            "[".yellow(),
            new_path.file_name().unwrap().to_string_lossy().italic(),
            "]".yellow()
        );
    } else {
        let msg = format!(
            "{} {} {}{}{} {} {}{}{} {}\n{} {} {}{}{}\n{} {} {}{}{}",
            "[❓]".dimmed(),
            "Do you really want to replace all".red().bold(),
            "[ \'".yellow(),
            replace_char,
            "\' ]".yellow(),
            "with".red().bold(),
            "[ \'".yellow(),
            new_char,
            "\' ]".yellow(),
            "in the file? (y/n)".red().bold(),
            " ↪  ".dimmed(),
            "Old name:".dimmed(),
            "[ \'".yellow(),
            path.file_name()
                .unwrap()
                .to_string_lossy()
                .italic()
                .dimmed(),
            "\' ]".yellow(),
            " ↪  ".dimmed(),
            "New name:".bright_green(),
            "[ \'".yellow(),
            new_path.file_name().unwrap().to_string_lossy().italic(),
            "\' ]".yellow(),
        );

        if confirm(&msg) {
            fs::rename(path, new_path)?;
            println!(
                "{} {}\n{} {} {} {} {}\n{} {} {} {} {}",
                "[✔] ".dimmed(),
                "Successfully renamed".green().bold(),
                " ↪  ".dimmed(),
                "Old name:".dimmed(),
                "[".yellow(),
                path.file_name()
                    .unwrap()
                    .to_string_lossy()
                    .italic()
                    .dimmed(),
                "]".yellow(),
                " ↪  ".dimmed(),
                "New name:".bright_green(),
                "[".yellow(),
                new_path.file_name().unwrap().to_string_lossy().italic(),
                "]".yellow()
            );
        } else {
            println!("{} {}", "[❌]".dimmed(), "Nevermind then".bold().dimmed());
        }
    }

    Ok(())
}

// TODO
fn show_lower_or_upper_name(path: &Path, uppercase_flag: bool, lowercase_flag: bool) -> String {
    println!("TODO");
    String::new()
}

// TODO
fn make_lower_or_upper(path: &Path, new_path: &Path, all_flag: bool) -> io::Result<()> {
    println!("TODO");
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
