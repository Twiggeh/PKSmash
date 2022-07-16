pub mod pokemon;
pub mod sop;
pub mod user;
use mongodb::{Client, Database};
use thiserror::Error;
use tokio::sync::OnceCell;

static DB_CONNECTION: OnceCell<Client> = OnceCell::const_new();

#[derive(Error, Debug)]
pub enum Err {
    #[error("error in database {0:#?}")]
    DbErr(#[from] mongodb::error::Error),
}

pub async fn ensure_index<T>(
    collection: mongodb::Collection<T>,
    index: mongodb::IndexModel,
) -> Result<Option<mongodb::results::CreateIndexResult>, Err> {
    if let Ok(mut indices) = collection.list_indexes(None).await {
        let name = index.options.as_ref().unwrap().name.as_ref().unwrap();
        while indices.advance().await? {
            let curr = indices.deserialize_current()?;
            if let Some(true) = curr
                .options
                .as_ref()
                .and_then(|ops| ops.name.as_ref().map(|x| x.as_str() == name))
            {
                return Ok(None);
            }
            // println!("{:#?}", curr);
        }
    }
    // TODO: Conditionally call this
    Ok(Some(collection.create_index(index, None).await?))
}

pub async fn populate_once_cell() {
    let client = Client::with_uri_str("mongodb://0.0.0.0").await.unwrap();

    DB_CONNECTION.set(client).unwrap();
}

pub fn get_db_client() -> Client {
    DB_CONNECTION.get().unwrap().clone()
}
pub fn get_db() -> Database {
    get_db_client().database("local")
}
