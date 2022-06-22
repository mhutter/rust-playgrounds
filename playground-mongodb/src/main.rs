use futures::TryStreamExt;
use mongodb::{
    bson::{doc, from_bson, Bson, Document, RawDocumentBuf},
    options::ClientOptions,
    Client, Database,
};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    let client = connect().await?;
    let db = client.default_database().unwrap();

    so72696316(db).await?;

    Ok(())
}

async fn connect() -> mongodb::error::Result<Client> {
    let client = {
        let mut options =
            ClientOptions::parse("mongodb://localhost:27017/local?appName=rust-playground-mongodb")
                .await?;
        options.app_name = Some(env!("CARGO_CRATE_NAME").to_string());
        Client::with_options(options)?
    };

    client
        .database("admin")
        .run_command(doc! {"ping":1}, None)
        .await?;
    println!("Connection successful!");

    Ok(client)
}

#[derive(Debug, Serialize, Deserialize)]
struct Book {
    author: String,
    title: String,
}

async fn so72696316(db: Database) -> mongodb::error::Result<()> {
    seed_so72696316(db.clone()).await?;
    use rayon::prelude::*;

    let coll = db.collection("books");
    let docs: Vec<RawDocumentBuf> = coll.find(None, None).await?.try_collect().await?;

    let z: Vec<Book> = docs
        .par_iter()
        .map(|x| {
            let doc = x.to_document().unwrap();
            from_bson(Bson::Document(doc))
        })
        .collect();

    dbg!(&z);
    assert_eq!(z.len(), 3);

    Ok(())
}

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
