mod backup;
mod monitor;
mod notification;
mod schedule;
mod select;

use dialoguer::{theme::ColorfulTheme, Select};
use std::process;

fn main() {
    // Seleção dos diretórios de monitoramento e backup
    let source_dir = select::select_directory("Select the directory to monitor");
    let backup_dir = select::select_directory("Select the backup directory");
    println!("Monitoring directory: {}", source_dir);
    println!("Backup will be saved to: {}", backup_dir);

    // Seleção do intervalo de backup
    let cron_expression = schedule::select_backup_interval();
    println!("Backup programado!");

    // Agendamento do backup com base no intervalo selecionado
    schedule::schedule_backup(&source_dir, &backup_dir, &cron_expression);

    // Início do monitoramento do diretório
    monitor::watch_directory(&source_dir, &backup_dir);
}

