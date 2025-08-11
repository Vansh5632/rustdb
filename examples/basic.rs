use std::result;

use rust_db::{Database,Schema,DbError};
use serde::{Serialize,Deserialize};

#[derive(Debug,Serialize,Deserialize,Schema)]

struct User{
    #[index]
    id:u64,
    name:String,
    email:String,
    age:u32,
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

    println!("users over 25: {:?}",result);

    Ok(())
}