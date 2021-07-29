use mongodb::bson::{self, doc};
use std::error::Error;
use tokio;
use mongodb::options::ClientOptions;
use mongodb::Collection;
use bson::Document;
use serde::Serialize;

#[derive(Serialize)]
struct TestData {
    pub(crate) my_date: bson::DateTime,
    pub(crate) my_decimal: bson::Decimal128,
    pub(crate) my_string: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client_uri = "mongodb://127.0.0.1:27017";
    let options = ClientOptions::parse(&client_uri).await?;
    let client = mongodb::Client::with_options(options)?;
    let collection: Collection<Document> = client.database("test").collection("test");

    let data = TestData {
        my_date: bson::DateTime::from_millis(1278720000000),
        my_decimal: bson::Decimal128::from_u32(42),
        my_string: "foobar".to_string()
    };
    let doc = bson::to_document(&data).unwrap();

    collection.insert_one(doc, None).await.unwrap();

    Ok(())
}