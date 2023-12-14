use std::env::VarError;



#[derive(Debug)]
pub enum Error {
    WrongPassword,
    Unauthorized,
    CannotDecryptToken,
    EnvironmentVariable(VarError)
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            Error::CannotDecryptToken => write!(f, "Cannot decrypt token error!"),
            Error::Unauthorized => write!(f, "Unathorized access error!"),
            Error::WrongPassword => write!(f, "Wrong password error!"),
            Error::EnvironmentVariable(e) => write!(f, "Error parsing environment variable: {:?}", e)
        }
    }
}