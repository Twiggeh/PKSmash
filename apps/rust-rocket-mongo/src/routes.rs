mod auth;
pub mod pksmash;

use rocket::{Build, Rocket};

use crate::api_types::{ApiResponseError, ApiResult};

#[catch(default)]
fn catcher() -> ApiResult<()> {
    ApiResult::new_err(ApiResponseError::NotFound, None)
}

pub fn mount(rocket: Rocket<Build>) -> Rocket<Build> {
    self::pksmash::mount(self::auth::mount(rocket)).register("/", catchers![catcher])
}
