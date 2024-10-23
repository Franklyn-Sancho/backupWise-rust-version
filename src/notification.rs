use notify_rust::Notification;

pub fn send_backup_notification(file_path: &str) {
    Notification::new()
        .summary("Backup Completed")
        .body(&format!("The file {} has been successfully saved!", file_path))
        .icon("dialog-information") // Default icon
        .show()
        .unwrap();
}

