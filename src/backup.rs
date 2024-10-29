use std::fs;
use std::path::Path;

use crate::notification::send_backup_notification;


pub fn backup_file(src: &str, dest: &str) {
    let source = Path::new(src);
    let destination = Path::new(dest).join(source.file_name().unwrap());

    // Check if the source is a file before attempting to copy
    if source.is_file() {
        // Copy the file to the destination
        fs::copy(&source, &destination).expect("Error copying file");
        
        // Send backup notification for the copied file
        send_backup_notification(&destination.to_string_lossy());
    } else {
        // Log an error if the source file does not exist
        eprintln!("File not found for backup: {:?}", source);
    }
}

pub fn backup_directory(src: &str, dest: &str) {
    let source_dir = Path::new(src);
    let destination_dir = Path::new(dest);

    // Check if the source is a directory before attempting to back it up
    if source_dir.is_dir() {
        // Iterate over the entries in the source directory
        for entry in fs::read_dir(source_dir).expect("Error reading directory") {
            let entry = entry.expect("Error getting directory entry");
            let source_file = entry.path();
            let destination_file = destination_dir.join(entry.file_name());

            // Copy files from source directory to destination directory
            if source_file.is_file() {
                fs::copy(&source_file, &destination_file).expect("Error copying file");
            }
        }
        send_backup_notification(src);
    } else {
        eprintln!("Source directory not found: {:?}", source_dir);
    }
}


