use mongodb::{bson::oid::ObjectId, results::InsertOneResult, IndexModel};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SmashOrPass {
    #[serde(rename = "_id")]
    pub object_id: ObjectId,

    pub user_id: ObjectId,
    pub pokemon_id: u32,

    pub smash: bool, // true=smash, false=pass
}

impl SmashOrPass {
    pub fn new(user_id: ObjectId, pokemon_id: u32, smash: bool) -> Self {
        Self {
            object_id: ObjectId::new(),
            user_id,
            pokemon_id,
            smash,
        }
    }

    fn collection() -> mongodb::Collection<Self> {
        let db = super::get_db();

        db.collection::<Self>("pksmash")
    }

    pub async fn index() -> Result<Option<mongodb::results::CreateIndexResult>, super::Err> {
        let mut options = mongodb::options::IndexOptions::default();
        let mut index = IndexModel::default();
        todo!()

        // options.sparse = Some(true);
        // options.unique = Some(true);
        // options.name = Some("usernameIndex".into());

        // index.keys = mongodb::bson::doc! {
        //     "username": 1
        // };
        // index.options = Some(options);

        // super::ensure_index(Self::collection(), index).await
    }
    pub async fn get_by_user_and_pid(
        user_id: ObjectId,
        pokemon_id: u32,
    ) -> Result<Option<Self>, super::Err> {
        let col = Self::collection();

        Ok(col
            .find_one(
                mongodb::bson::doc! {"user_id":user_id,"pokemon_id":pokemon_id },
                None,
            )
            .await?)
    }

    pub async fn save(&self) -> Result<InsertOneResult, mongodb::error::Error> {
        let col = Self::collection();

        col.insert_one(self, None).await
    }
}
