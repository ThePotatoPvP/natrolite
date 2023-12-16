use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BasicCommandData {
    // text might be an empty string, this struct will be used for commands that need a minimal input
    pub text: String,
}