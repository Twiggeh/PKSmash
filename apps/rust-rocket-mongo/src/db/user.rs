use bcrypt::hash;
use mongodb::bson::oid::ObjectId;
use mongodb::results::InsertOneResult;
use mongodb::IndexModel;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Clone, Deserialize)]
pub struct PublicUser {
    #[serde(rename = "id")]
    pub object_id: ObjectId,

    pub username: String,
    pub displayname: String,
}

impl From<User> for PublicUser {
    fn from(user: User) -> Self {
        PublicUser {
            object_id: user.object_id,
            username: user.username,
            displayname: user.displayname,
        }
    }
}

#[derive(Serialize, Debug, Clone, Deserialize, PartialEq)]
pub struct User {
    #[serde(rename = "_id")]
    pub object_id: ObjectId,
    pub username: String,
    pub displayname: String,
    pub password_hash: String,
}

impl User {
    pub fn new(
        username: String,
        displayname: String,
        password: &str,
    ) -> Result<Self, bcrypt::BcryptError> {
        let password_hash = hash(password, 10)?;

        Ok(Self {
            object_id: ObjectId::new(),
            username,
            displayname,
            password_hash,
        })
    }

    fn collection() -> mongodb::Collection<User> {
        let db = super::get_db();

        db.collection::<Self>("user")
    }

    pub async fn index() -> Result<Option<mongodb::results::CreateIndexResult>, super::Err> {
        let mut options = mongodb::options::IndexOptions::default();
        let mut index = IndexModel::default();

        options.sparse = Some(true);
        options.unique = Some(true);
        options.name = Some("usernameIndex".into());

        index.keys = mongodb::bson::doc! {
            "username": 1
        };
        index.options = Some(options);

        super::ensure_index(Self::collection(), index).await
    }

    pub async fn get_by_uname(uname: String) -> Result<Option<Self>, super::Err> {
        let col = Self::collection();

        Ok(col
            .find_one(mongodb::bson::doc! { "username": uname }, None)
            .await?)
    }
    pub fn passwords_match<'a>(&self, pw: &'a str) -> bool {
        bcrypt::verify(pw, &self.password_hash).unwrap()
    }

    pub async fn get_by_uname_and_pw<'a>(
        uname: String,
        pw: &'a str,
    ) -> Result<Option<Self>, super::Err> {
        Ok(Self::get_by_uname(uname).await?.and_then(move |user| {
            if user.passwords_match(pw) {
                Some(user)
            } else {
                None
            }
        }))
    }

    pub async fn save(&self) -> Result<InsertOneResult, mongodb::error::Error> {
        let col = Self::collection();

        col.insert_one(self, None).await
    }
}
