pub mod error;
pub mod token;

use std::{convert::Infallible, ops::FromResidual};

pub use error::*;
use rocket::{http, Response};
use serde::Serialize;

use self::token::Token;

pub trait Status {
    fn get_status(&self) -> u16;
}

#[derive(Serialize)]
#[serde(tag = "type")]
pub(crate) enum ApiResult<D, E: Status = error::ApiResponseError> {
    Ok {
        #[serde(with = "crate::api_types::token::serde")]
        token: Option<Token>,
        data: D,
    },
    Err {
        #[serde(with = "crate::api_types::token::serde")]
        token: Option<Token>,
        error: E,
    },
}

impl<D, E: Status> ApiResult<D, E> {
    pub fn new(data: D, token: Option<Token>) -> Self {
        Self::Ok { token, data }
    }
    pub fn new_err(error: E, token: Option<Token>) -> Self {
        Self::Err { token, error }
    }

    pub fn status(&self) -> u16 {
        match self {
            ApiResult::Ok { token, data } => 200,
            ApiResult::Err { token, error } => error.get_status(),
        }
    }

    fn get_token(&self) -> &Option<Token> {
        match self {
            ApiResult::Ok { ref token, data } => token,
            ApiResult::Err { ref token, error } => token,
        }
    }
    fn set_token(&mut self, t: Option<Token>) {
        match self {
            ApiResult::Ok {
                ref mut token,
                data,
            } => *token = t,
            ApiResult::Err {
                ref mut token,
                error,
            } => *token = t,
        }
    }
}

impl<D, E: Status, ID: Into<D>, IE: Into<E>> From<Result<ID, IE>> for ApiResult<D, E> {
    fn from(x: Result<ID, IE>) -> Self {
        match x {
            Ok(ok) => Self::new(ok.into(), None),
            Err(err) => Self::new_err(err.into(), None),
        }
    }
}

impl<D, E: Status, IE: Into<E>> FromResidual<Result<Infallible, IE>> for ApiResult<D, E> {
    fn from_residual(residual: Result<Infallible, IE>) -> Self {
        Self::Err {
            token: None,
            error: residual.unwrap_err().into(),
        }
    }
}

impl<'r, 'o: 'r, D: Serialize, E: Status + Serialize> rocket::response::Responder<'r, 'o>
    for ApiResult<D, E>
{
    fn respond_to(mut self, request: &'r rocket::Request<'_>) -> rocket::response::Result<'o> {
        if self.get_token().is_none() {
            self.set_token(Token::create_from_request(request).and_then(Token::renew));
        }

        let string = serde_json::to_string(&self).map_err(|_| http::Status::InternalServerError)?;

        let mut response = Response::new();

        response.set_header(http::ContentType::JSON);
        response.set_status(
            http::Status::from_code(self.status()).ok_or(http::Status::InternalServerError)?,
        );
        response.set_sized_body(string.len(), std::io::Cursor::new(string));

        Ok(response)
    }
}
