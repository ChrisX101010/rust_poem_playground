use poem::http::StatusCode;
use poem::Error as PoemError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("user not found")]
    NotFound,
    #[error("bad request")]
    BadRequest,
}

impl From<MyError> for PoemError {
    fn from(err: MyError) -> PoemError {
        match err {
            MyError::NotFound => PoemError::from_status(StatusCode::NOT_FOUND),
            MyError::BadRequest => PoemError::from_status(StatusCode::BAD_REQUEST),
        }
    }
}
