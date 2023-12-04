use serde::{Serialize, Deserialize};
use serde_email::{Email, is_valid_email, EmailError};


#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct User {
    user_id: UserId,
    email: Email,
    name: String,
    password: String,
    role: String,
}

#[derive(Debug,Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct UserId(pub usize);

// impl Serialize for UserID {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//         where
//             S: serde::Serializer {
//         serializer.serialize_newtype_struct("user_id", &self.0)
//     }
// }

pub fn validate_email(email: String) -> Result<Email, EmailError> {
    if is_valid_email(&email) {
        let email = Email::from_string(email).unwrap();
        return Ok(email);
    }
    let invalid_email = EmailError::Invalid { raw_email: email };
    Err(invalid_email)
}