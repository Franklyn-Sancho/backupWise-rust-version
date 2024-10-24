mod backup;
mod monitor;
mod notification;
mod schedule;
mod select;

fn main() {
    let source_dir = select::select_directory("Select the directory to monitor");
    let backup_dir = select::select_directory("Select the backup directory");

    println!("Monitoring directory: {}", source_dir);
    println!("Backup will be saved to: {}", backup_dir);

    // Agendamento de 1 em 1 minuto
    let cron_expression = "1 * * * * *";

    schedule::schedule_backup(&source_dir, &backup_dir, cron_expression);

    // Continue o monitoramento normal...
    monitor::watch_directory(&source_dir, &backup_dir);
}
