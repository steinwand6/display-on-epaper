use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    template_path: String,
    display_image_path: String,
    ttf_path: String,
    task_file_path: String,
}

impl Config {
    pub fn get_config() -> Self {
        let config = include_str!("../config.toml");
        toml::from_str(config).unwrap()
    }

    pub fn get_template(&self) -> &str {
        &self.template_path
    }

    pub fn get_ttf(&self) -> &str {
        &self.ttf_path
    }

    pub fn get_diplay_image(&self) -> &str {
        &self.display_image_path
    }

    pub fn get_task_file(&self) -> &str {
        &self.task_file_path
    }
}
