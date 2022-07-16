use std::error::Error;

use serde::Serialize;

pub struct AuthContainer(String);
//pub struct UserContainer(String);

impl<T> FnOnce<(Option<T>,)> for AuthContainer {
    type Output = Result<T, ApiResponseError>;

    extern "rust-call" fn call_once(self, (args,): (Option<T>,)) -> Self::Output {
        args.ok_or(ApiResponseError::AuthError { content: self.0 })
    }
}

#[derive(Serialize)]
pub enum ApiResponseError {
    AuthError { content: String },
    PokemonError { content: String },
    NotFound,
}

impl super::Status for ApiResponseError {
    fn get_status(&self) -> u16 {
        match self {
            ApiResponseError::AuthError { content } => 400,
            ApiResponseError::PokemonError { content } => 400,
            ApiResponseError::NotFound => 404,
        }
    }
}

impl ApiResponseError {
    pub fn new_auth(content: String) -> Self {
        Self::AuthError { content }
    }
    pub fn into_auth(e: impl Error) -> Self {
        Self::AuthError {
            content: format!("{}", e),
        }
    }
    pub fn into_pokemon(e: impl Error) -> Self {
        Self::PokemonError {
            content: format!("{}", e),
        }
    }
    //    pub fn into_user(e: impl Error) -> Self {
    //        Self::AuthError {
    //            content: format!("{}", e),
    //        }
    //    }
    pub fn auth_result_builder(s: String) -> AuthContainer {
        AuthContainer(s)
    }

    //   pub fn user_result_builder(s: String) -> UserContainer {
    //       UserContainer(s)
    //   }
}
