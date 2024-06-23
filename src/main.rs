mod error;
mod routes;

use poem::{
    get, handler, http::StatusCode, listener::TcpListener, web::Path, Error, Result, Route, Server,
};
use routes::create_routes;

#[handler]
fn hello(Path(name): Path<String>) -> Result<String> {
    if name.is_empty() {
        Err(Error::from_status(StatusCode::BAD_REQUEST))
    } else {
        Ok(format!("Hello, {}!", name))
    }
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Create the route for the /hello/:name endpoint
    let hello_route = Route::new().at("/hello/:name", get(hello));

    // Retrieve the routes defined in routes.rs using create_routes
    let api_routes = create_routes();

    // Combine the routes without nesting one inside the other
    let app = Route::new().nest("/api", api_routes).nest_no_strip("/", hello_route);

    // Start the server on port 3000
    Server::new(TcpListener::bind("0.0.0.0:3000")).run(app).await
}
