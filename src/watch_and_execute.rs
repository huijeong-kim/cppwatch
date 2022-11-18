use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
use std::ffi::OsStr;
use std::path::PathBuf;
use std::process::{Child, Command};
use std::sync::mpsc::{Receiver, Sender};
use std::time::Duration;

pub fn run(path: PathBuf, cmd: String) -> Result<(), Box<dyn std::error::Error>> {
    let command = get_command_to_execute(&path, cmd);

    let (tx, rx) = std::sync::mpsc::channel();

    // Watcher -(tx, rx)- check_changes_thread
    // check_changes_trhead -(tx, rx)- run_command_thread

    println!("Watch {}", path.display());
    let mut watcher = watcher(tx, Duration::from_secs(1)).expect("Failed to setup folder watcher");
    watcher
        .watch(path, RecursiveMode::Recursive)
        .expect("Failed to watch src folder");

    // Start check changes thread
    let (tx_trigger, rx_executor) = std::sync::mpsc::channel();
    let check_changes_thread = std::thread::spawn(|| {
        on_src_changes(rx, tx_trigger);
    });

    // Start command execution thread
    let run_command_thread = std::thread::spawn(|| {
        run_command(command, rx_executor);
    });

    check_changes_thread.join().unwrap();
    run_command_thread.join().unwrap();

    Ok(())
}

fn on_src_changes(rx: Receiver<DebouncedEvent>, tx_trigger: Sender<DebouncedEvent>) {
    while let Ok(event) = rx.recv() {
        if is_triggering_cmd_execution(&event) {
            tx_trigger.send(event).unwrap();
        }
    }
}

fn is_triggering_cmd_execution(event: &DebouncedEvent) -> bool {
    match event {
        DebouncedEvent::Create(file)
        | DebouncedEvent::Write(file)
        | DebouncedEvent::Rename(_, file) => {
            let filename = file
                .file_name()
                .unwrap_or(OsStr::new("invalid"))
                .to_str()
                .unwrap_or("invalid");

            println!("{:?}, file {}", &event, filename);

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

fn run_command(cmd: String, rx: Receiver<DebouncedEvent>) {
    let mut child: Option<Child> = None;

    while let Ok(_event) = rx.recv() {
        kill_prev_child(child.take());

        println!("Execute: {}", cmd);
        child = Some(
            Command::new("sh")
                .arg("-c")
                .arg(cmd.clone())
                .spawn()
                .expect("Failed to execute command"),
        );

        if child.is_some() {
            if let Ok(Some(status)) = child.as_mut().unwrap().try_wait() {
                println!("Result: {:?}", status);
            }
        }
    }
}

fn kill_prev_child(prev_child: Option<Child>) {
    if prev_child.is_some() {
        println!("Kill previous run process");
        prev_child.unwrap().kill().unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use notify::watcher;
    use std::fs;
    use std::fs::File;
    use std::time::Duration;

    #[test]
    fn watcher_should_notify_event_when_file_created() {
        // TODO test fails intermittently
        let test_dir = PathBuf::from("./watcher_should_notify_event_when_file_created");
        fs::remove_dir_all(&test_dir);
        fs::create_dir(&test_dir).unwrap();

        // Start watcher watching test_path
        let (tx, rx) = std::sync::mpsc::channel();
        let mut watcher = watcher(tx, Duration::from_secs(1)).unwrap();
        watcher.watch(&test_dir, RecursiveMode::Recursive).unwrap();

        // Start event receiver thread
        let event = std::thread::spawn(move || -> Option<DebouncedEvent> {
            let mut event = None;
            while let Ok(e) = rx.recv() {
                println!("{:?}", e);
                event = Some(e);
            }
            return event;
        });

        // Create test file
        let mut test_file = test_dir.clone();
        test_file.push("test_file");
        File::create(&test_file).unwrap();

        std::thread::sleep(Duration::from_secs(3));
        drop(watcher);

        let expected_event = DebouncedEvent::Create(fs::canonicalize(&test_file).unwrap());
        assert_eq!(event.join().unwrap().unwrap(), expected_event);

        fs::remove_dir_all(&test_dir).unwrap();
    }
}
