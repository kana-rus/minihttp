use minihttp::{Request, Response, JSON, Server, ServerResult};
use serde::Serialize;


#[derive(Serialize)]
struct User {
    id:   usize,
    name: String,
}

fn main() -> ServerResult {
    Server::setup()
        .GET("/", show_user_info)
        .POST("/", post_articles)
        .serve("127.0.0.1:3000") 
}

fn show_user_info(_req: Request) -> Response {
    let user = User {id: 1, name: "first user".into()};
    Response::OK(
        JSON::from_struct(&user)
    )
}
fn post_articles(_req: Request) -> Response {
    Response::NotFound()
}
