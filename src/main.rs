mod backup;
mod monitor;
mod notification;
mod schedule;
mod select;
mod cron;

use backupwise_rs::persistence;


fn main() {
    let persistence = persistence::Persistence::load();
    println!("Monitoring directory: {}", persistence.source_dir);
    println!("Backup will be saved to: {}", persistence.backup_dir);

    let cron_expression = persistence.cron_expression();
    println!("Backup programado para cada {} minuto(s)", persistence.backup_interval);

    schedule::schedule_backup(&persistence.source_dir, &persistence.backup_dir, &cron_expression);
    monitor::watch_directory(&persistence.source_dir, &persistence.backup_dir);
}



