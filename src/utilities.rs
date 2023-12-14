use std::env;
use dotenv::dotenv;
use handle_errors::Error;


// return environment variables
pub fn get_env_variable(variable: &str) -> Result<String, handle_errors::Error> {
    dotenv().ok();

    match env::var(variable) {
        Ok(var) => Ok(var),
        Err(e) => 
            { 
                eprintln!("Error parsing environment_variable: {:?}", e); 
                Err(Error::EnvironmentVariable(e))
            }
    }
}
