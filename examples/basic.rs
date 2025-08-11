use rust_db::{Database,Schema,DbError};
use serde::{Serialize,Deserialize};

#[derive(Debug,Serialize,Deserialize)]
struct User{
    #[allow(dead_code)]
    id:u64,
    name:String,
    email:String,
    age:u32,
}

impl Schema for User {
    fn schema_validate(&self) -> Result<(), rust_db::SchemaError> {
        // Add validation logic here if needed
        Ok(())
    }
    
    fn table_name() -> &'static str {
        "User"
    }
}

#[tokio::main]

async fn main()-> Result<(),DbError>{
    pretty_env_logger::init();
    let db = Database::open("./data").await?;

    let user = User{
        id:1,
        name:"Alice".to_string(),
        email: "check".to_string(),
        age:30,
    };

    db.insert(&user).await?;
    println!("inserted user: {:?}", user);
    let results = db.query::<User>().filter(|u| u.age>25).execute().await?;

    println!("users over 25: {:?}",results);

    Ok(())
}