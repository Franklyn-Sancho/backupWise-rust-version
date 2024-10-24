use notify_rust::Notification;

// Notificação de backup
pub fn send_backup_notification(directory: &str) {
    Notification::new()
        .summary("Backup Completed")
        .body(&format!("All files in {} have been successfully backed up!", directory))
        .icon("dialog-information")
        .show()
        .unwrap();
}

