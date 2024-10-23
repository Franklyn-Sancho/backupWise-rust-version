mod monitor;
mod backup;
mod select;
mod notification;

fn main() {
    // Use the directory selection function from the select module
    let source_dir = select::select_directory("Select the directory to monitor");
    let backup_dir = select::select_directory("Select the backup directory");

    // Display the selected directories
    println!("Monitoring directory: {}", source_dir);
    println!("Backup will be saved in: {}", backup_dir);

    // Start monitoring and backup
    monitor::watch_directory(&source_dir, &backup_dir);
}

