use std::fs;
use std::path::Path;

use crate::notification::send_backup_notification;

pub fn backup_file(src: &str, dest: &str) {
    let source = Path::new(src);
    let destination = Path::new(dest).join(source.file_name().unwrap());
    // Check if the file exists and copy
    if source.is_file() {
        fs::copy(&source, &destination).expect("Error copying file");
        println!("Backup completed: {:?}", destination);
        send_backup_notification(&destination.to_string_lossy());
    }
}

pub fn backup_directory(src: &str, dest: &str) {
    let source_dir = Path::new(src);
    let destination_dir = Path::new(dest);

    if source_dir.is_dir() {
        for entry in fs::read_dir(source_dir).expect("Error reading directory") {
            let entry = entry.expect("Error getting directory entry");
            let source_file = entry.path();
            let destination_file = destination_dir.join(entry.file_name());

            if source_file.is_file() {
                fs::copy(&source_file, &destination_file).expect("Error copying file");
                println!("Backup completed for: {:?}", source_file);
            }
        }

        // Envia uma notificação única para todos os arquivos do diretório
        send_backup_notification(src);
    }
}
