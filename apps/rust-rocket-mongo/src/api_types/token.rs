use ::serde::{Deserialize, Serialize};
use chrono::{serde::ts_seconds, DateTime, Duration, Utc};
use jsonwebtoken::{
    decode, encode, errors, Algorithm, DecodingKey, EncodingKey, Header, Validation,
};
use rocket::{request::Outcome, Request};

use crate::db::user::PublicUser;

#[derive(Serialize, Deserialize, Clone)]
pub struct Token {
    #[serde(with = "ts_seconds")]
    pub exp: DateTime<Utc>,

    #[serde(flatten)]
    pub user: PublicUser,
}

impl Token {
    pub const fn get_secret() -> &'static [u8] {
        let s = "secret";

        s.as_bytes()
    }

    pub fn new(user: PublicUser) -> Self {
        Self {
            user,
            exp: Utc::now() + Duration::days(3),
        }
    }

    pub fn renew(self) -> Option<Self> {
        self.should_renew().then(move || Self::new(self.user))
    }
    pub fn should_renew(&self) -> bool {
        Utc::now() - self.exp > Duration::days(7)
    }

    pub fn encode(self) -> errors::Result<String> {
        encode(
            &Header::default(),
            &self,
            &EncodingKey::from_secret(Self::get_secret()),
        )
    }
    pub fn decode(string: String) -> errors::Result<jsonwebtoken::TokenData<Token>> {
        decode(
            &string,
            &DecodingKey::from_secret(Self::get_secret()),
            &Validation::new(Algorithm::HS256),
        )
    }
    pub fn create_from_request<'r>(request: &'r Request<'_>) -> Option<Self> {
        request
            .headers()
            .get_one("Authorization")
            .and_then(|x| {
                if x.starts_with("Basic ") {
                    Some(x.replace("Basic ", ""))
                } else {
                    None
                }
            })
            .and_then(|x| Token::decode(x).ok())
            .map(|x| x.claims)
    }
}

#[rocket::async_trait]
impl<'r> rocket::request::FromRequest<'r> for Token {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let auth_header = Self::create_from_request(request);

        match auth_header {
            Some(v) => Outcome::Success(v),
            None => Outcome::Forward(()),
        }
    }
}

pub mod serde {
    pub fn serialize<S>(
        maybe_token: &Option<super::Token>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        match maybe_token {
            Some(token) => serializer.serialize_str(&token.clone().encode().unwrap()),
            None => serializer.serialize_none(),
        }
    }
}
