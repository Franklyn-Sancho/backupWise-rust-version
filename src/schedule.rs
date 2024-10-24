use cron::Schedule;
use std::str::FromStr;
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


