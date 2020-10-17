#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
use mongodb::{Client};
use rocket_contrib::json::Json;
use rocket::response::status::NotFound;

#[get("/")]
async fn index() ->  String {
    
    "Hello, world!".to_string()
}
#[get("/dbs")]
async fn test() -> Result<Json<Vec<String>>, NotFound<String>>{    
    match get_db_names().await {
        Ok(db_names) => Ok(Json(db_names)),
        Err(e) => Err(NotFound(e.to_string()))
    }
}

async fn get_db_names() -> Result<Vec<String>, mongodb::error::Error>{
    let client = Client::with_uri_str("mongodb://localhost:27017/").await?;
    Ok(client.list_database_names(None, None).await?)
}


// allows asynchronous main method
#[tokio::main]
  async fn  main() {
      //.await on the end of launch to allow async routes
    rocket::ignite().mount("/", routes![index,test]).launch().await;
}