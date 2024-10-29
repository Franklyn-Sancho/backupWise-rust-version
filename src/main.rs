mod backup;
mod monitor;
mod notification;
mod schedule;
mod select;
mod cron;
mod logger;
mod driver_integration;
mod config;
mod auth;

use std::{error::Error, sync::Arc};

use auth::authenticate;
use backupwise_rs::persistence;
use logger::log_event;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    log_event("Inicialização", "Programa de monitoramento de backup iniciado.");

    // Autenticação com o Google Drive
    let hub = auth::authenticate().await?;
    let hub = Arc::new(hub);  // Torna o hub compartilhável

    let persistence = persistence::Persistence::load();
    println!("Monitoring directory: {}", persistence.source_dir);
    println!("Backup will be saved to: {}", persistence.backup_dir);
    let cron_expression = persistence.cron_expression();
    println!("Backup programado para cada {} minuto(s)", persistence.backup_interval);

    // Agendar backup e monitorar diretório
    schedule::schedule_backup(&persistence.source_dir, &persistence.backup_dir, &cron_expression);

    // Modifique a função `monitor::watch_directory` para aceitar o hub
    monitor::watch_directory(&persistence.source_dir, &persistence.backup_dir, &hub).await;

    log_event("Finalização", "Programa de monitoramento de backup encerrado.");
    Ok(())
}




