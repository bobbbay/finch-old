use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    theme: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            theme: "".to_string(),
        }
    }
}
