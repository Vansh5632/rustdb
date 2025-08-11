use rust_db::{Database, DbError};

// This should fail at compile time due to invalid table name
rust_db::schema! {
    table_name: "Invalid-Table-Name!",  // Contains invalid characters
    #[derive(Debug)]
    struct InvalidUser {
        id: u64,
        name: String,
    }
}

rust_db::impl_basic_schema!(InvalidUser, "Invalid-Table-Name!");

#[tokio::main]
async fn main() -> Result<(), DbError> {
    println!("This should not compile!");
    Ok(())
}
