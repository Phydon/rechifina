use clap::Command;
use colored::*;
use flexi_logger::{detailed_format, Duplicate, FileSpec, Logger};
use log::{error, info, warn};

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
    match matches.subcommand() {
        Some(("run", _)) => {
            if let Err(err) = replace_chars() {
                error!("Error executing cmds: {}", err);
                process::exit(1);
            }
        }
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

fn rechifina() -> Command {
    Command::new("rechifina")
        .about("Replace char in filename")
        .version("1.0.0")
        .author("Leann Phydon <leann.phydon@gmail.com")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("log")
                .short_flag('l')
                .about("Show content of the log file"),
        )
        .subcommand(Command::new("run").short_flag('r').about("Run"))
}

fn replace_chars() -> io::Result<()> {
    info!("Started");
    let msg = "Do you really want to replace the chars in the given filenames? (y/n)";
    if confirm(msg) {
        warn!("About to replace chars in filenames");
    } else {
        println!("{}", "Nevermind then".dimmed());
    }

    Ok(())
}

fn confirm(msg: &str) -> bool {
    loop {
        println!("{}", msg.yellow().bold());

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
