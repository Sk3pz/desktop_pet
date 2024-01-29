use std::fs;
use serde::Deserialize;
use crate::{DATA_PATH, pet_handler};

#[derive(Debug, Clone, Deserialize)]
pub struct PetConfigSection {
    pub name: String,
    pub pet_type: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub(crate) pet: PetConfigSection,
}

impl Config {
    fn create_default() -> std::io::Result<()> {
        let valid_pets = pet_handler::PetType::valid_pets();

        let default_config = format!("[pet]\
        \nname = \"pet\"\
        \n# valid types: {}\
        \npet_type = \"dog\"",
            valid_pets.join(", "));

        fs::create_dir_all(DATA_PATH)?;
        fs::write(format!("{}/config.toml", DATA_PATH), default_config)?;

        Ok(())
    }

    pub fn load() -> Self {
        if !fs::metadata(format!("{}/config.toml", DATA_PATH)).is_ok() {
            // todo: better error handling in case of config creation failure
            Self::create_default().unwrap();
        }

        let config = fs::read_to_string(format!("{}/config.toml", DATA_PATH)).unwrap();
        toml::from_str(&config).unwrap()
    }
}