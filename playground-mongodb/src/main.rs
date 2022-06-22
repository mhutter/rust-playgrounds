use futures::TryStreamExt;
use mongodb::{
    bson::{self, doc, Document, RawDocumentBuf},
    options::ClientOptions,
    Client, Database,
};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    let client = connect().await?;
    let db = client.default_database().unwrap();

    so72696316(db).await
}

async fn connect() -> mongodb::error::Result<Client> {
    let client = {
        let mut options =
            ClientOptions::parse("mongodb://localhost:27017/local?appName=rust-playground-mongodb")
                .await?;
        options.app_name = Some(env!("CARGO_CRATE_NAME").to_string());

        // for more options see:
        // https://docs.rs/mongodb/latest/mongodb/options/struct.ClientOptions.html

        Client::with_options(options)?
    };

    // Ping the DB to confirm we have a connection
    client
        .database("admin")
        .run_command(doc! {"ping": 1}, None)
        .await?;
    println!("---> Connection successful!");

    Ok(client)
}

/// A sample struct to use in our program
#[derive(Debug, Serialize, Deserialize)]
struct Book {
    author: String,
    title: String,
}

async fn so72696316(db: Database) -> mongodb::error::Result<()> {
    use rayon::prelude::*;
    seed_so72696316(db.clone()).await?;
    let coll = db.collection("books");

    // Fetch all documents but do not deserialize them yet
    let docs: Vec<RawDocumentBuf> = coll.find(None, None).await?.try_collect().await?;

    // Deserialize all documents using Rayon's parallel iterator
    let z: Vec<Book> = docs
        .par_iter()
        .map(|raw| bson::from_slice(raw.as_bytes()).unwrap())
        .collect();

    dbg!(&z);
    assert_eq!(z.len(), 3);

    Ok(())
}

/// Seed the "books" collection with some entries if it is empty.
async fn seed_so72696316(db: Database) -> mongodb::error::Result<()> {
    let books = db.collection::<Document>("books");

    let n = books.count_documents(None, None).await?;
    if n < 1 {
        let docs = vec![
            doc! { "title": "1984", "author": "George Orwell" },
            doc! { "title": "Animal Farm", "author": "George Orwell" },
            doc! { "title": "The Great Gatsby", "author": "F. Scott Fitzgerald" },
        ];
        books.insert_many(docs, None).await?;
    }
    Ok(())
}
