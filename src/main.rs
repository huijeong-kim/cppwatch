use clap::{arg, value_parser, Command};
use std::path::PathBuf;

mod watch_and_execute;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TODO use logger

    let (path, cmd) = get_values().unwrap();
    watch_and_execute::run(path, cmd)?;

    Ok(())
}

fn get_values() -> Result<(PathBuf, String), Box<dyn std::error::Error>> {
    let matches = Command::new("BuildWatcher")
        .version("0.1")
        .arg(
            arg!(-s --src <PATH> "Src code location to watch")
                .required(true)
                .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            arg!(-x --execute <CMD> "Command to execute when src changed")
                .required(true)
                .value_parser(value_parser!(String)),
        )
        .get_matches();

    // TODO enable command cascading e.g., -x "make -j12" -x "make run_ut"

    let path = matches
        .get_one::<PathBuf>("src")
        .expect("Wrong src path provided");
    let cmd = matches
        .get_one::<String>("execute")
        .expect("Wrong command provided");

    Ok((path.to_owned(), cmd.to_owned()))
}
