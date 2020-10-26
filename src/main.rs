#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
use rocket::response::status::NotFound;
use rocket::State;
use rocket_contrib::json::Json;
mod expense_service;

#[get("/")]
async fn index() -> String {
    "Hello, world!".to_string()
}

#[get("/expenses")]
async fn expenses_all(expense_dao: State<'_,expense_service::ExpenseDAO>) -> Result<Json<Vec<expense_service::Expense>>, NotFound<String>> {
    match expense_dao.get_all_expenses().await {
        Ok(expenses) => Ok(Json(expenses)),
        Err(e) => Err(NotFound(e.to_string())),
    }
}
#[get("/expenses/<id>")]
async fn expense_by_id(expense_dao: State<'_,expense_service::ExpenseDAO>, id: String) -> Result<Json<expense_service::Expense>, NotFound<String>> {
    match expense_dao.get_expense_by_id(id).await {
        Ok(expenses) => Ok(Json(expenses)),
        Err(e) => Err(NotFound(e.to_string())),
    }
}
#[post("/expenses", data = "<expense>")]
async fn create_expense(expense_dao: State<'_,expense_service::ExpenseDAO>, expense: Json<expense_service::Expense>) -> Result<Json<String>, NotFound<String>> {
    match expense_dao.create_expense(expense.0).await {
        Ok(result) => Ok(Json(result)),
        Err(e) => Err(NotFound(e.to_string())),
    }
}
// allows asynchronous main method
// #[tokio::main]
//   async fn  main() {
//       //.await on the end of launch to allow async routes
//     rocket::ignite().mount("/", routes![index,test]).launch().await;
// }
#[launch]
async fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![index, expenses_all,create_expense, expense_by_id])
    .manage(expense_service::ExpenseDAO::new().await.expect("shit"))
}
