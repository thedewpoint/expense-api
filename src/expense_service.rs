
use futures::stream::StreamExt;
use mongodb::{Client, bson, bson::doc, bson::oid::ObjectId, error::Result};
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct Expense {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    name: String,
    recurrance: String,
    amount: u8
 } 
 pub struct ExpenseDAO<'a> {
    client: Client,
    db_name: &'a str,
    collection: &'a str
 }
 impl<'a> ExpenseDAO<'a> {
      pub async fn new()  -> Result<ExpenseDAO<'a>> {
        Ok(ExpenseDAO {
            client : Client::with_uri_str("mongodb://localhost:27017/").await?,
            db_name : "finance",
            collection : "expense"
        })
     }
     pub async fn get_all_expenses(&self) -> Result<Vec<Expense>>{
         let expenses = self.client.database(self.db_name).collection(self.collection);
         let mut results = expenses.find(None,None).await?;
         let mut result_vec: Vec<Expense> = Vec::new();
         while let Some(doc) = results.next().await {
            let expense: Expense = bson::from_bson(bson::Bson::Document(doc?))?;
            result_vec.push(expense);
         }
         Ok(result_vec)
    }
    pub async fn get_expense_by_id(&self,id: String) -> Result<Expense>{
         let expenses = self.client.database(self.db_name).collection(self.collection);
         println!("id is {}",id);
         let doc = expenses.find_one(doc! {
            "_id" : ObjectId::with_string(id.as_str()).expect("something went wrong")
         },
         None).await?;
         let expense : Expense  = bson::from_bson(bson::Bson::Document(doc.unwrap()))?;
         Ok(expense)
   }
 }

