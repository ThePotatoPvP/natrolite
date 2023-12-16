use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AimedCommandData {
    // TODO implement special values for things likes @everyone or @here or things like that.
    pub id: i64,
}