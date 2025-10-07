use anyhow::Result;
use exec::{Command, Error};
use log;
use regex::Regex;
use std::env;
use std::ffi::OsString;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::process::exit;
use systemd_journal_logger::JournalLog;

fn setup_logging() {
    JournalLog::new().unwrap().install().unwrap();
    log::set_max_level(log::LevelFilter::Info);
}

fn lines(path: OsString) -> Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let lines: std::result::Result<Vec<String>, std::io::Error> = reader.lines().collect();
    Ok(lines?)
}

fn run_command(command: &str) -> Error {
    Command::new("sh").arg("-c").arg(command).exec()
}

fn fail() -> ! {
    println!("denied");
    exit(1)
}

fn main() {
    setup_logging();
    let rules_file = match env::args_os().nth(1) {
        Some(path) => path,
        None => {
            log::error!("No rules file argument recieved");
            fail()
        }
    };
    let original_command = match env::var("SSH_ORIGINAL_COMMAND") {
        Ok(value) => value,
        Err(e) => {
            log::error!("Cannot retrive original command: {e}");
            fail()
        }
    };
    let rules = match lines(rules_file) {
        Ok(v) => v,
        Err(e) => {
            log::error!("Error reading rules file: {e}");
            fail()
        }
    };
    log::info!("Orignal command: {original_command}");
    for rule in rules {
        // Skip comments
        if rule.trim_start().starts_with("#") {
            continue;
        }
        match Regex::new(&rule) {
            Ok(r) => {
                if r.is_match(&original_command) {
                    log::info!("Matched rule {rule}, running command");
                    let err = run_command(&original_command);
                    log::error!("Error running command: {err}");
                    println!("error");
                    exit(1);
                }
            }
            Err(e) => {
                log::error!("Invalid regex in rules file: {rule}: {e}");
                fail();
            }
        }
    }
    log::info!("Command did not match any rule");
    fail()
}
