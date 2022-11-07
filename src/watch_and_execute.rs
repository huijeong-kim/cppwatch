use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
use std::ffi::OsStr;
use std::path::PathBuf;
use std::process::Command;
use std::sync::mpsc::Receiver;
use std::time::Duration;

pub fn run(path: PathBuf, cmd: String) -> Result<(), Box<dyn std::error::Error>> {
    let (tx, rx) = std::sync::mpsc::channel();

    let command = get_command_to_execute(&path, cmd);

    // Watch source folder
    println!("Watch {}", path.display());
    let mut watcher = watcher(tx, Duration::from_secs(1)).unwrap();
    watcher.watch(path, RecursiveMode::Recursive).unwrap();

    handle_changes(rx, command);

    Ok(())
}

fn handle_changes(rx: Receiver<DebouncedEvent>, build_command: String) {
    while let Ok(event) = rx.recv() {
        if is_triggering_cmd_execution(event) {
            // TODO Stop if new changes come in
            run_command(build_command.clone());
        }
    }
}

fn is_triggering_cmd_execution(event: DebouncedEvent) -> bool {
    match event {
        DebouncedEvent::Create(file)
        | DebouncedEvent::Write(file)
        | DebouncedEvent::Rename(_, file) => {
            let filename = file
                .file_name()
                .unwrap_or(OsStr::new("invalid"))
                .to_str()
                .unwrap_or("invalid");
            println!("filename: {}", filename);
            is_src_file(&filename)
        }
        _ => false,
    }
}

fn is_src_file(filename: &str) -> bool {
    filename.ends_with(".cpp")
        || filename.ends_with(".h")
        || filename.ends_with(".hpp")
        || filename.ends_with(".c")
}

fn get_command_to_execute(path: &PathBuf, cmd: String) -> String {
    format!("cd {}; {}", path.display(), cmd)
}

fn run_command(cmd: String) {
    // TODO printout result to terminal
    println!("Execute: {}", cmd);

    let output = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .output()
        .expect("Failed to execute command");

    println!("Result: {:?}", output.stdout);
}
