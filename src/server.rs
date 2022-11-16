use std::{
    collections::HashMap,
    net::TcpListener,
    io::{Write, Read}
};
use crate::{
    result::ServerResult,
    request::{Method, Request},
    response::Response,
    components::{
        consts::BUF_SIZE,
    }
};


pub struct Server(
    HashMap<(Method, &'static str), fn(Request) -> ServerResult<Response>>
); impl Server {
    pub fn setup() -> Self {
        Self(HashMap::new())
    }
    pub fn serve(&mut self, address: &'static str) -> ServerResult<()> {
        let listener = TcpListener::bind(address)?;
        for stream in listener.incoming() {
            let mut stream = stream?;
            let mut buffer = [0; BUF_SIZE];

            stream.read(&mut buffer)?;
            let (path, method, request) = parse_stream(&buffer)?;

            println!("requested: {:?} {}", method, path);

            let response = 'res: {
                let Some(handler) = self.0.get(&(method, path)) else {
                    break 'res Response::NotFound()
                };
                match handler(request) {
                    Ok(res)  => res,
                    Err(res) => res,
                }
            };
            response.write_to_stream(&mut stream)?;
            stream.flush()?
        }
        Ok(())
    }
    #[allow(non_snake_case)]
    pub fn GET(&mut self, path: &'static str, handler: fn(Request) -> ServerResult<Response>) -> &mut Self {
        self.resister_handler(Method::GET, path, handler)
    }
    #[allow(non_snake_case)]
    pub fn POST(&mut self, path: &'static str, handler: fn(Request) -> ServerResult<Response>) -> &mut Self {
        self.resister_handler(Method::POST, path, handler)
    }

    fn resister_handler(&mut self,
        method: Method,
        path:   &'static str,
        handler: fn(Request) -> ServerResult<Response>,
    ) -> &mut Self {
        assert!(path.starts_with("/"), "endpoint path '{path}' doesn't start with '/' !");
        let duplication_panic_message = format!("handler for '{:?} {path}' is already resistered !", method);
        let duplication = self.0.insert((method, path), handler);
        assert!(duplication.is_none(), "{duplication_panic_message}");
        self
    }
}


fn parse_stream(buffer: &[u8; BUF_SIZE]) -> ServerResult<(&str, Method, Request)> {
    let request_status = {
        let mut end_of_reqest_status = BUF_SIZE;
        for pos in 0..BUF_SIZE {
            if buffer[pos]   == b'\r'  
            && buffer[pos+1] == b'\n' {
                if pos == 0 {
                    return Err(Response::BadRequest())//"HTTP request starts with '\\r'"
                }
                end_of_reqest_status = pos - 1;
                break
            }
        }
        if end_of_reqest_status == BUF_SIZE {
            return Err(Response::BadRequest())//"HTTP request doesn't contain any valid request status".into()))
        }
        &buffer[..=end_of_reqest_status]
    };

    let mut split = request_status.split(|b| *b == b' ');
    let method = match split.next().expect("no method found in request") {
        b"GET"  => Method::GET,
        b"POST" => Method::POST,
        _ => return Err(Response::BadRequest())//"HTTP request doesn't contain any valid method"
    };
    let path = std::str::from_utf8(
        split.next().expect("no request path found in request")
    ).expect("failed to get path from buffer");

    Ok((
        path,
        method,
        Request {
            // headers: vec![],
            body: None,
        }
    ))
}