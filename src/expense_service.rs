use futures::stream::StreamExt;
use mongodb::{bson, bson::doc, bson::oid::ObjectId, error::Result, Client, Collection};
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct Expense {
   #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
   id: Option<ObjectId>,
   name: String,
   recurrance: String,
   amount: u8,
}
pub struct ExpenseDAO {
   collection: Collection,
}
impl ExpenseDAO {
   pub async fn new() -> Result<ExpenseDAO> {
      let client = Client::with_uri_str("mongodb://localhost:27017/").await?;
      let collection = client.database("finance").collection("expense");
      Ok(ExpenseDAO {
         collection: collection,
      })
   }
   pub async fn get_all_expenses(&self) -> Result<Vec<Expense>> {
      let mut results = self.collection.find(None, None).await?;
      let mut result_vec: Vec<Expense> = Vec::new();
      while let Some(doc) = results.next().await {
         let expense: Expense = bson::from_bson(bson::Bson::Document(doc?))?;
         result_vec.push(expense);
      }
      Ok(result_vec)
   }
   pub async fn get_expense_by_id(&self, id: String) -> Result<Expense> {
      let doc = self
         .collection
         .find_one(
            doc! {
               "_id" : ObjectId::with_string(id.as_str()).expect("something went wrong")
            },
            None,
         )
         .await?;
      let expense: Expense = bson::from_bson(bson::Bson::Document(doc.unwrap()))?;
      Ok(expense)
   }
   pub async fn create_expense(&self, expense: Expense) -> Result<String> {
      let expense_bson = bson::to_bson(&expense)?;
      let document = expense_bson.as_document().unwrap();
      let result = self.collection.insert_one(document.to_owned(), None).await?;
      Ok(result.inserted_id.as_str().unwrap().to_owned())
   }
}
