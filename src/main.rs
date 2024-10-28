mod backup;
mod monitor;
mod notification;
mod schedule;
mod select;
mod cron;
mod logger;

use backupwise_rs::persistence;
use logger::log_event;


fn main() {
    log_event("Inicialização", "Programa de monitoramento de backup iniciado.");

    let persistence = persistence::Persistence::load();
    println!("Monitoring directory: {}", persistence.source_dir);
    println!("Backup will be saved to: {}", persistence.backup_dir);

    let cron_expression = persistence.cron_expression();
    println!("Backup programado para cada {} minuto(s)", persistence.backup_interval);

    schedule::schedule_backup(&persistence.source_dir, &persistence.backup_dir, &cron_expression);
    monitor::watch_directory(&persistence.source_dir, &persistence.backup_dir);

    log_event("Finalização", "Programa de monitoramento de backup encerrado.");
}




