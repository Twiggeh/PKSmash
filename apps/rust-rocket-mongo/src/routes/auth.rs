use rocket::{serde::json::Json, Build, Rocket};
use serde::Deserialize;

use crate::{
    api_types::{token::Token, ApiResponseError, ApiResult},
    db::user::{PublicUser, User},
};

#[derive(Deserialize)]
struct AuthReqData {
    username: String,
    password: String,
}

#[post("/sign_up", data = "<data>", format = "json")]
async fn sign_up(data: Json<AuthReqData>) -> ApiResult<PublicUser> {
    let data = data.into_inner();

    let u = User::new(data.username.clone(), data.username, &data.password)
        .map_err(ApiResponseError::into_auth)?;
    u.save().await.map_err(ApiResponseError::into_auth)?;

    let user = PublicUser::from(u);
    ApiResult::new(user.clone(), Some(Token::new(user)))
}

#[post("/sign_in", data = "<data>", format = "json")]
async fn sign_in(data: Json<AuthReqData>) -> ApiResult<PublicUser> {
    let data = data.into_inner();

    let user: PublicUser = User::get_by_uname_and_pw(data.username, &data.password)
        .await
        .map_err(ApiResponseError::into_auth)
        .and_then(ApiResponseError::auth_result_builder("".into()))?
        .into();

    ApiResult::new(user.clone(), Some(Token::new(user)))
}

pub fn mount(rocket: Rocket<Build>) -> Rocket<Build> {
    // This does not error if registers sign_in twice
    rocket.mount("/auth", routes![sign_up, sign_in])
}
