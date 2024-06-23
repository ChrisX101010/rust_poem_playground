use poem::{
    error::ParseQueryError, handler, http::StatusCode, web::Query, IntoResponse, Response, Result,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Params {
    name: String,
}

#[handler]
fn index(res: Result<Query<Params>>) -> Result<impl IntoResponse> {
    match res {
        Ok(Query(params)) => Ok(params.name.into_response()),
        Err(err) if err.is::<ParseQueryError>() => {
            Ok(Response::builder().status(StatusCode::INTERNAL_SERVER_ERROR).body(err.to_string()))
        }
        Err(err) => Err(err),
    }
}
