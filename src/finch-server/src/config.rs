use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub theme: String,
    pub db_url: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            theme: "".to_string(),
            db_url: "".to_string(),
        }
    }
}
