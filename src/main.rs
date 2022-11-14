use minihttp::{Request, Response, JSON, Server, ServerResult};


fn main() -> ServerResult {
    Server::setup()
        .GET("/", show_articles)
        .POST("/", post_articles)
        .serve("127.0.0.1:3000") 
}

fn show_articles(_req: Request) -> Response {
    Response::OK(JSON())
}
fn post_articles(_req: Request) -> Response {
    Response::NotFound()
}
