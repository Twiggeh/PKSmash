#![feature(decl_macro, proc_macro_hygiene)]
#![feature(async_closure)]
#![feature(try_trait_v2)]
#![feature(unboxed_closures)]
#![feature(fn_traits)]
#![feature(is_some_with)]

/// Error : {
///   type: 'UserError',
///   content,
/// } | {
///   type: 'PokemonError',
///   content,
/// } | {
///   type: 'SecurityError',
///   content,
/// }
///
/// User : {
///   displayname,
///   username,
///   createdAt,
///   id,
/// } + Password for priv
///
/// UserPokemonResult : {
///   userId,
///   pokemonId,
///   smash: bool,
/// }
///
/// Comment : {
///   authorId,
///   pokemonId,
///   content,
///   childrenIds,
/// }
///
/// Pokemon : {
///   totalSmash,
///   totalPass,
///   views,
/// }
///
/// JWT Content = Entire User
///
/// /auth/sign_up -> user + JWT:auth-header
/// /auth/sign_in + JWT:auth-header -> user +
///
/// /queue/get_next + JWT:auth-header -> Pokemon
///
/// # smash or pass
/// /{pokemonId}/sop/submit + JWT + body:UserPokemonResult -> Pokemon
/// /{pokemonId}/sop/result + JWT -> UserPokemonResult
/// /sop/result + JWT -> UserPokemonResult[]
///
/// /{pokemonId}/data -> { data: Pokemon comments: Comment[] }
pub(crate) mod api_types;
pub(crate) mod db;
mod middleware;
mod routes;

use rocket::{figment::Figment, Build, Rocket};
use thiserror::Error;

use crate::middleware::cors::CORS;

#[macro_use]
extern crate rocket;

fn construct_figment() -> Figment {
    Figment::from(rocket::Config::default())
        .merge(("log_level", rocket::config::LogLevel::Debug))
        .merge((
            rocket::Config::PORT,
            std::env::var("PORT")
                .unwrap_or("8000".to_string())
                .parse::<u16>()
                .unwrap(),
        ))
        .merge((rocket::Config::ADDRESS, "0.0.0.0"))
}

pub fn build_rocket() -> Rocket<Build> {
    let figment = construct_figment();

    let rocket = rocket::custom(figment).attach(CORS);
    let rocket = routes::mount(rocket);

    rocket
}

#[derive(Error, Debug)]
enum BootError {
    #[error("Rocket error {0:#?}")]
    RocketError(#[from] rocket::Error),

    #[error("Internal database error {0:#?}")]
    DbErr(#[from] crate::db::Err),

    #[error("Mongo DB Error {0:#?}")]
    MongoErr(#[from] mongodb::error::Error),

    #[error("Internal io error {0:#?}")]
    IoErr(#[from] std::io::Error),
}

#[rocket::main]
async fn main() -> Result<(), BootError> {
    crate::db::populate_once_cell().await;
    db::user::User::index().await?;

    let r = build_rocket();

    let _rocket = r.launch().await?;

    Ok(())
}
