use serde::{Serialize, Deserialize};



#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
    user_id: usize,
    session_token: String,
}