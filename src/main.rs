use minihttp::{Request, Response, JSON, Server, Context};
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
struct User {
    id:   usize,
    name: String,
}

fn main() -> Context<()> {
    Server::setup()
        .GET("/", show_user_info)
        .POST("/", post_new_user)
        .serve_on(":3000") 
}

fn show_user_info(_req: Request) -> Context<Response> {
    let user = User {id: 1, name: "first user".to_owned()};
    Response::OK(
        JSON::from_struct(&user)?
    )
}
fn post_new_user(req: Request) -> Context<Response> {
    let Some(new_user) = req.get_body::<User>()? else {
        return Response::BadRequest("request 'POST /' has to have user json as its request body")
    };

    // handle DB ...

    let created = new_user;

    Response::OK(
        JSON::from_struct(&created)?
    )
}
