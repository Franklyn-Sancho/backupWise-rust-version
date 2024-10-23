use std::process::Command;

pub fn select_directory(prompt: &str) -> String {
    let output = Command::new("zenity")
        .arg("--file-selection")
        .arg("--directory")
        .arg("--title")
        .arg(prompt)
        .output()
        .expect("Failed to execute Zenity");

    if output.status.success() {
        let directory = String::from_utf8_lossy(&output.stdout);
        return directory.trim().to_string(); // Remove extra spaces and newlines
    } else {
        panic!("Error selecting the directory");
    }
}

