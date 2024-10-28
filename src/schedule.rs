use crate::backup::backup_directory;
use chrono::Utc;
use cron::Schedule;
use dialoguer::{theme::ColorfulTheme, Select};
use std::sync::{Arc, Mutex};
use std::thread;
use std::{process, str::FromStr};
use crate::logger::log_event;

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
                        log_event("Backup Agendado", &format!("Backup iniciado em: {}", Utc::now()));
                        backup_directory(&src_path, &dest_path);
                        log_event("Backup Concluído", "Backup agendado concluído com sucesso.");
                    }
                }
            });
        }
        Err(e) => eprintln!("Error parsing cron expression: {:?}", e),
    }
}


pub fn select_backup_interval() -> String {
    println!("Selecionando o intervalo de backup...");
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

    {
        let interval = match selection {
            0 => 1,
            1 => 5,
            2 => 10,
            3 => 30,
            4 => 60,
            _ => 1,
        };
        match interval {
            1 => "1 * * * * *".to_string(),
            5 => "5 * * * * *".to_string(),
            10 => "10 * * * * *".to_string(),
            30 => "30 * * * * *".to_string(),
            60 => "* 1 * * * *".to_string(),
            _ => "1 * * * * *".to_string(),
        }
    }
}
