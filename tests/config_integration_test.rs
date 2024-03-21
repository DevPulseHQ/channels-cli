use std::process::Command;
use std::str;

#[test]
fn test_configuration_from_file() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("--config")
        .arg("tests/test_config.yml")
        .output()
        .expect("Failed to execute command");

    // Check if the command was successful
    if !output.status.success() {
        // Convert stdout and stderr from Vec<u8> to &str for printing
        let stdout = str::from_utf8(&output.stdout).unwrap_or("Failed to read stdout");
        let stderr = str::from_utf8(&output.stderr).unwrap_or("Failed to read stderr");

        // Print stdout and stderr
        println!("Command failed with status: {:?}", output.status);
        println!("stdout: {}", stdout);
        println!("stderr: {}", stderr);

        // Assert failure after printing
        assert!(false, "Command did not execute successfully");
    }
}
