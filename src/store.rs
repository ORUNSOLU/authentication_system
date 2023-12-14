use once_cell::sync::Lazy;
use serde::{Serialize, Deserialize};
// use std::borrow::Cow;
use surrealdb::Surreal;
use surrealdb::opt::auth::Root;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::engine::remote::ws::Client;
// use serde_email::Email;

use crate::utilities::get_env_variable;
use crate::models::users::{User, validate_email, Roles};



#[derive(Serialize, Deserialize)]
struct DbConnection;


// initialize the connection on a static client
static DB: Lazy<Surreal<Client>> = Lazy::new(Surreal::init);

impl DbConnection {
    pub async fn initiate_connection() -> surrealdb::Result<()> {
        let address = get_env_variable("EndpointURL").unwrap();
        DB.connect::<Ws>(address.as_str()).await?;

        let username = get_env_variable("USERNAME").unwrap();
        let password = get_env_variable("PASSWORD").unwrap();
        DB.signin( Root {
            username: username.as_str(),
            password: password.as_str(),
        }).await?;

        let namespace = get_env_variable("NAMESPACE").unwrap();
        let db_name = get_env_variable("DATABASE").unwrap();
        DB.use_ns(namespace.as_str()).use_db(db_name.as_str()).await?;
        Ok(())
    }

    pub async fn create_user(&self, email: String, name: String, pass: String, role: String) -> surrealdb::Result<()> {
        let confirmed_email = validate_email(email).unwrap();

        let new_user = User {
            email: confirmed_email.clone(),
            name: name,
            password: pass,
            role: role
        };

        // email is used as the ID
        let _:Option<User> = DB.create(("users", confirmed_email.as_str()))
            .content(new_user)
            .await?;

        Ok(())
    }

    pub async fn update_user(&self) -> surrealdb::Result<()> {
        todo!("Update user function")
    }
}
