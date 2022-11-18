use std::{
    collections::HashMap,
    net::TcpListener,
    io::{Write, Read},
};
use crate::{
    result::Context,
    request::{Method, Request},
    response::Response,
    components::{
        consts::{
            BUF_SIZE,
            ENB
        },
    }, JSON
};


pub struct Server(
    HashMap<(Method, &'static str), fn(Request) -> Context<Response>>
); impl Server {
    pub fn setup() -> Self {
        Self(HashMap::new())
    }
    pub fn serve_on(&mut self, address: &'static str) -> Context<()> {
        let address =
            if address.starts_with(":") {"127.0.0.1".to_owned() + address} else
            if address.starts_with("localhost") {address.replace("localhost", "127.0.0.1")} else {
                address.to_owned()
            };
        let listener = TcpListener::bind(address)?;
        for stream in listener.incoming() {
            let mut stream = stream?;
            let mut buffer = [ENB; BUF_SIZE]; // String ::with_capacity(BUF_SIZE); // [ENB; BUF_SIZE];

            stream.read(&mut buffer)?;
            let (method, path, request) = parse_stream(&mut buffer)?;

            println!("requested: {:?} {}", method, path);

            let response = 'res: {
                let Some(handler) = self.0.get(&(method, path)) else {
                    break 'res Response::NotFound::<String, ()>(format!("handler for that request is not found")).unwrap_err()
                };
                match handler(request) {
                    Ok(res)  => res,
                    Err(res) => res,
                }
            };
            // ======================================================================================
            // println!("about to respond: {:?}", response); 
            response.write_to_stream(&mut stream)?;
            // println!("fin: response.write_to_stream()");
            stream.flush()?;
            // println!("fin: stream.flush()");
            // ======================================================================================
        }
        Ok(())
    }
    #[allow(non_snake_case)]
    pub fn GET(&mut self, path: &'static str, handler: fn(Request) -> Context<Response>) -> &mut Self {
        self.resister_handler(Method::GET, path, handler)
    }
    #[allow(non_snake_case)]
    pub fn POST(&mut self, path: &'static str, handler: fn(Request) -> Context<Response>) -> &mut Self {
        self.resister_handler(Method::POST, path, handler)
    }

    fn resister_handler(&mut self,
        method:  Method,
        path:    &'static str,
        handler: fn(Request) -> Context<Response>,
    ) -> &mut Self {
        assert!(path.starts_with("/"), "endpoint path '{path}' doesn't start with '/' !");
        let duplication_panic_message = format!("handler for '{:?} {path}' is already resistered !", method);
        let duplication = self.0.insert((method, path), handler);
        assert!(duplication.is_none(), "{duplication_panic_message}");
        self
    }
}


fn parse_stream(
    buffer: &[u8; BUF_SIZE]
    // buffer: &mut String
) -> Context<(Method, &str, Request)> {
    // buffer.shrink_to_fit();
    // let mut lines = buffer.split("\r\n"); // .lines();
    let mut lines = std::str::from_utf8(buffer)?
        .trim_end() // trim *" " made by `ENB`: b' ' in original buffer ( &[u8] )
        .lines();

    let request_line = lines.next().ok_or_else(|| Response::BadRequest::<&str, ()>("empty request").unwrap_err())?;
    let (method, path) = parse_request_line(request_line)?;

    // let mut debug_count = 0; // ==================
    while let Some(line) = lines.next() {
        // debug_count += 1;
        // println!("{debug_count}th loop");
        if line.is_empty() {break}
        // in current version, DON'T handle request headers
    }
    // println!("leaved from {debug_count}-count loop"); // ==================

    let request = Request {
        body: if let Some(request_body) = lines.next() {
            Some(JSON::from_string_unchecked(request_body.to_owned()))} else {None}
    };

    Ok((method, path, request))
}

fn parse_request_line(line: &str) -> Context<(Method, &str)> {
    if line.is_empty() {
        return Response::BadRequest("can't find request status line")
    }
    let (method, path) = line
        .strip_suffix(" HTTP/1.1")
        .ok_or_else(|| Response::NotImplemented::<&str, ()>("I can't handle protocols other than `HTTP/1.1`").unwrap_err())?
        .split_once(' ')
        .ok_or_else(|| Response::BadRequest::<&str, ()>("invalid request line format").unwrap_err())?;
    Ok((Method::parse(method)?, path))
}
