#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
use rocket_contrib::json::Json;
use rocket::response::status::NotFound;
mod expense_service;

#[get("/")]
async fn index() ->  String {
    
    "Hello, world!".to_string()
}

#[get("/expenses")]
async fn expenses_all() -> Result<Json<Vec<expense_service::Expense>>, NotFound<String>> {    
    let expense_dao : expense_service::ExpenseDAO = expense_service::ExpenseDAO::new().await.expect("shit");
     match expense_dao.get_all_expenses().await {
        Ok(expenses) => Ok(Json(expenses)),
        Err(e) => Err(NotFound(e.to_string()))
    }
}
#[get("/expenses/<id>")]
async fn expense_by_id(id: String) -> Result<Json<expense_service::Expense>, NotFound<String>> {    
    let expense_dao : expense_service::ExpenseDAO = expense_service::ExpenseDAO::new().await.expect("shit");
     match expense_dao.get_expense_by_id(id).await {
        Ok(expenses) => Ok(Json(expenses)),
        Err(e) => Err(NotFound(e.to_string()))
    }
}
// allows asynchronous main method
// #[tokio::main]
//   async fn  main() {
//       //.await on the end of launch to allow async routes
//     rocket::ignite().mount("/", routes![index,test]).launch().await;
// }
#[launch]
fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![index, expenses_all, expense_by_id])
}