use minihttp::{Request, Response, JSON, Server, ServerResult};
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
struct User {
    id:   usize,
    name: String,
}

fn main() -> ServerResult<()> {
    Server::setup()
        .GET("/", show_user_info)
        .POST("/", post_new_user)
        .serve("127.0.0.1:3000") 
}

fn show_user_info(_req: Request) -> ServerResult<Response> {
    let user = User {id: 1, name: "first user".into()};
    Ok(Response::OK(
        JSON::from_struct(&user)?
    ))
}
fn post_new_user(req: Request) -> ServerResult<Response> {
    let Some(new_user) = req.get_body::<User>()? else {
        return Err(Response::BadRequest())
    };

    // handle DB ...

    let created = new_user;

    Ok(Response::OK(
        JSON::from_struct(&created)?
    ))
}
