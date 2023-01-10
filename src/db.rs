use diesel::{insert_into, prelude::*, query_builder::InsertStatement, query_dsl::methods::ExecuteDsl};
use std::env;

pub fn connect() -> MysqlConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
