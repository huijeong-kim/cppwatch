use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::time::Duration;

#[test]
fn command_should_be_executed_when_cpp_file_created() {
    let test_dir = PathBuf::from("./command_should_be_executed_when_cpp_file_created");
    fs::remove_dir_all(&test_dir);
    fs::create_dir(&test_dir).unwrap();

    let mut test_file = test_dir.clone();
    test_file.push("test_file.cpp");

    let command = format!("echo hi there > {:?}", test_file.file_name().unwrap());

    let watch_dir = test_dir.clone();
    std::thread::spawn(|| {
        cppwatch::watch_and_execute::run(watch_dir, command).unwrap();
    });


    File::create(&test_file).expect("Failed to create test file");
    std::thread::sleep(Duration::from_secs(5));

    let mut file = File::open(&test_file).expect("Failed to open test file");
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();

    assert_eq!(buf, "hi there\n");

    fs::remove_dir_all(test_dir).unwrap();
}
