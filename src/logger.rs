use chrono::Utc;
use serde_json::json;
use std::fs::{create_dir_all, OpenOptions};
use std::io::Write;

pub fn log_event(status: &str, details: &str) {
    let log_entry = json!({
        "status": status,
        "details": details,
        "timestamp": Utc::now().to_string(),
    });

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("backup_log.json")
        .expect("Não foi possível abrir o log principal");

    writeln!(file, "{}", log_entry.to_string()).expect("Erro ao escrever no log principal");
}

pub fn log_file_change(file_path: &str, change_type: &str, details: &str) {
    let log_dir = "file_logs";
    create_dir_all(log_dir).expect("Erro ao criar diretório de logs de arquivos");

    let file_name = format!("{}/{}.json", log_dir, file_path.replace("/", "_"));
    let log_entry = json!({
        "change_type": change_type,
        "details": details,
        "timestamp": Utc::now().to_string(),
    });

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(&file_name)
        .expect("Não foi possível abrir o log específico do arquivo");

    writeln!(file, "{}", log_entry.to_string()).expect("Erro ao escrever no log específico do arquivo");
}
