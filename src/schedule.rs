use cron::Schedule;
use dialoguer::{theme::ColorfulTheme, Select};
use std::{process, str::FromStr};
use std::thread;
use std::time::Duration;
use chrono::Utc;
use crate::backup::backup_file;  // Supondo que já exista uma função de backup
use std::sync::{Arc, Mutex};
use crate::backup::backup_directory;


pub fn schedule_backup(src: &str, dest: &str, cron_expression: &str) {
    match Schedule::from_str(cron_expression) {
        Ok(schedule) => {
            let src = Arc::new(Mutex::new(String::from(src)));
            let dest = Arc::new(Mutex::new(String::from(dest)));

            thread::spawn(move || {
                for datetime in schedule.upcoming(Utc) {
                    let now = Utc::now();
                    if datetime > now {
                        let duration_until_next_backup = (datetime - now).to_std().unwrap();
                        thread::sleep(duration_until_next_backup);

                        let src_path = src.lock().unwrap().clone();
                        let dest_path = dest.lock().unwrap().clone();
                        println!("Scheduled backup running at: {}", Utc::now());
                        backup_directory(&src_path, &dest_path); 
                    }
                }
            });
        },
        Err(e) => eprintln!("Error parsing cron expression: {:?}", e),
    }
}

pub fn select_backup_interval() -> String {
    // Opções de intervalo para backup
    let intervals = vec!["1 minuto", "5 minutos", "10 minutos", "30 minutos", "1 hora"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Escolha o intervalo para o backup automático")
        .items(&intervals)
        .default(0)
        .interact()
        .unwrap_or_else(|_| {
            println!("Nenhuma opção selecionada. Saindo...");
            process::exit(1);
        });

    // Expressão cron com base na seleção
    match selection {
        0 => "1 * * * * *".to_string(),   // A cada 1 minuto
        1 => "5 * * * * *".to_string(),   // A cada 5 minutos
        2 => "10 * * * * *".to_string(),  // A cada 10 minutos
        3 => "30 * * * * *".to_string(),  // A cada 30 minutos
        4 => "* 1 * * * *".to_string(),     // A cada 1 hora
        _ => "*/1 * * * *".to_string(),   // Valor padrão, caso algo falhe
    }
}


