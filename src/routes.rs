use poem::{get, handler, http::StatusCode, post, web::Path, Error, Result, Route};

#[handler]
async fn get_user(Path(id): Path<String>) -> Result<String> {
    match id.as_str() {
        "1" => Ok(format!("Fetching user with ID 1: John Doe")),
        "2" => Ok(format!("Fetching user with ID 2: Jane Smith")),
        _ => Err(Error::from_status(StatusCode::NOT_FOUND)),
    }
}

#[handler]
async fn delete_user(Path(id): Path<String>) -> Result<String> {
    match id.as_str() {
        "1" => Ok(format!("Deleting user with ID 1: John Doe")),
        "2" => Ok(format!("Deleting user with ID 2: Jane Smith")),
        _ => Err(Error::from_status(StatusCode::NOT_FOUND)),
    }
}

#[handler]
async fn create_user() -> Result<String> {
    Ok(format!("User created successfully"))
}

pub fn create_routes() -> Route {
    Route::new().at("/user/:id", get(get_user).delete(delete_user)).at("/user", post(create_user))
}
