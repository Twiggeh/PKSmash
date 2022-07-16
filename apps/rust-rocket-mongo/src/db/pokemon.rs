use mongodb::bson::doc;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Pokemon {
    #[serde(rename = "id")]
    pub object_id: u32,

    pub totalsmash: u32,
    pub totalpass: u32,
    pub views: u32,
}

impl Pokemon {
    pub fn new(object_id: u32) -> Self {
        Self {
            object_id,
            totalsmash: 0,
            totalpass: 0,
            views: 0,
        }
    }
    fn collection() -> mongodb::Collection<Self> {
        let db = super::get_db();

        db.collection::<Self>("pokemons")
    }

    pub async fn inc_smash(
        object_id: u32,
    ) -> Result<mongodb::results::UpdateResult, mongodb::error::Error> {
        let collection = Self::collection();

        collection
            .update_one(
                doc! { "_id": object_id },
                doc! { "$inc": {"totalsmash": 1} },
                None,
            )
            .await
    }
    pub async fn inc_pass(
        object_id: u32,
    ) -> Result<mongodb::results::UpdateResult, mongodb::error::Error> {
        let collection = Self::collection();

        collection
            .update_one(
                doc! { "_id": object_id },
                doc! { "$inc": {"totalpass": 1} },
                None,
            )
            .await
    }
}
