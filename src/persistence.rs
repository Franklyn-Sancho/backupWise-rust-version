use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};
use std::path::Path;

use crate::cron;
use crate::{schedule, select};

#[derive(Serialize, Deserialize)]
pub struct Persistence {
    pub source_dir: String,
    pub backup_dir: String,
    pub backup_interval: u32, // Em minutos
}

impl Persistence {
    pub fn load() -> Self {
        let config_path = "config.json";
        if Path::new(config_path).exists() {
            let config_content = fs::read_to_string(config_path).expect("Erro ao ler o arquivo de configuração");
            serde_json::from_str(&config_content).expect("Erro ao parsear a configuração")
        } else {
            let cron_expression = schedule::select_backup_interval();
            Self::create_new(cron_expression)
        }
    }

    pub fn create_new(cron_expression: String) -> Self {
        let source_dir = select::select_directory("Select the directory to monitor");
        let backup_dir = select::select_directory("Select the backup directory");
        let backup_interval = cron::cron_to_interval(&cron_expression);
        let config = Persistence {
            source_dir,
            backup_dir,
            backup_interval,
        };
        config.save();
        config
    }
    

    pub fn save(&self) {
        let config_content = serde_json::to_string_pretty(self).expect("Erro ao serializar a configuração");
        fs::write("config.json", config_content).expect("Erro ao salvar o arquivo de configuração");
    }

    // Função auxiliar para converter intervalo em cron_expression
    pub fn cron_expression(&self) -> String {
        cron::interval_to_cron(self.backup_interval)
    }
}

