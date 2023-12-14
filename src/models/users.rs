use serde::{Serialize, Deserialize};
use serde_email::{Email, is_valid_email, EmailError};


#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct User {
    pub email: String,
    pub name: String,
    pub password: String,
    pub role: String,
}

#[allow(dead_code)]
pub fn validate_email(email: String) -> Result<String, EmailError> {
    if is_valid_email(&email) {
        let email = Email::from_string(email).unwrap();
        return Ok(email.to_string());
    }
    let invalid_email = EmailError::Invalid { raw_email: email };
    Err(invalid_email)
}

#[allow(dead_code)]
pub enum Roles {
    BusinessAnalyst,
    ProjectManager,
    ResourceManager,
    Sponsor,
    TeamMember
}