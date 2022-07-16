use rocket::{Build, Rocket};

use crate::{
    api_types::{token::Token, ApiResponseError, ApiResult},
    db::{pokemon::Pokemon, sop::SmashOrPass},
};

#[get("/<pid>/sop/submit/<smash>")]
async fn submit_sop(pid: u32, token: Token, smash: bool) -> ApiResult<SmashOrPass> {
    if smash {
        Pokemon::inc_smash(pid).await
    } else {
        Pokemon::inc_pass(pid).await
    }
    .map_err(ApiResponseError::into_pokemon)?;

    let result = SmashOrPass::new(token.user.object_id, pid, smash);

    result
        .save()
        .await
        .map_err(ApiResponseError::into_pokemon)?;

    ApiResult::new(result, Some(token))
}

#[get("/<pid>/sop/result")]
async fn get_sop_result(pid: u32, token: Token) -> ApiResult<Option<SmashOrPass>> {
    ApiResult::new(
        SmashOrPass::get_by_user_and_pid(token.user.object_id, pid)
            .await
            .map_err(ApiResponseError::into_pokemon)?,
        Some(token),
    )
}

// This is actually Infallible
#[get("/<_pid>/sop/result", rank = 1)]
async fn get_sop_result_fallback(_pid: u32) -> ApiResult<()> {
    ApiResult::new_err(ApiResponseError::new_auth("Unauthenticated".into()), None)
}

pub fn mount(rocket: Rocket<Build>) -> Rocket<Build> {
    // This does not error if registers sign_in twice
    rocket.mount(
        "/",
        routes![get_sop_result, get_sop_result_fallback, submit_sop],
    )
}
