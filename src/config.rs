use serde::{Serialize, Deserialize};
use std::fs;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub source_dir: String,
    pub backup_dir: String,
    pub backup_interval: u32, // Em minutos
}

impl Config {
    pub fn load() -> Self {
        let config_content = fs::read_to_string("config.json").expect("Erro ao ler o arquivo de configuração");
        serde_json::from_str(&config_content).expect("Erro ao parsear a configuração")
    }

    pub fn save(&self) {
        let config_content = serde_json::to_string_pretty(self).expect("Erro ao serializar a configuração");
        fs::write("config.json", config_content).expect("Erro ao salvar o arquivo de configuração");
    }
}
