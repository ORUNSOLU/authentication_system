use once_cell::sync::Lazy;
use serde::{Serialize, Deserialize};
use std::borrow::Cow;
use surrealdb::Surreal;
use surrealdb::opt::auth::Root;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::engine::remote::ws::Client;
use serde_email::Email;

use crate::models::users::{User, validate_email, UserId};



#[derive(Serialize, Deserialize)]
struct DbConnection;


// Creates a new static instance of the client
static DB: Lazy<Surreal<Client>> = Lazy::new(Surreal::init);

impl DbConnection {
    pub async fn initiate_connection() -> surrealdb::Result<()> {
        // let db = Surreal::new::<Ws>("localhost:8000")
        // .with_capacity(100)
        // .await?;

        // connect to the DB
        todo!("Add environment variable for connection string");
        DB.connect::<Ws>("localhost:8000").await?;

        DB.signin( Root {
            username: "admin",
            password: "admin"
        }).await?;

        DB.use_ns("development1").use_db("authdb").await?;
    }

    pub async fn create_user(&self, email: String, name: String, pass: String, user_id: usize, role: String) -> surrealdb::Result<()> {
        let confirmed_email = validate_email(email).expect("Error validating email.");
        let confirmed_userid = UserId(user_id);

        let new_user = User {
            user_id: confirmed_userid,
            email: confirmed_email,
            name: name,
            password: pass,
            role: role
        };

        DB.update(("users", "id of person to update"))
            .content(new_user).await?
    }
}
