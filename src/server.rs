use std::{
    collections::HashMap,
    net::TcpListener,
    io::{Write, Read}
};
use crate::{
    request::{Method, Request},
    response::Response,
    components::{
        consts::BUF_SIZE,
        result::{ServerResult, ServerError}
    }
};


pub struct Server(
    HashMap<
        &'static str,
        HashMap<Method, fn(Request) -> Response>,
    >,
); impl Server {
    pub fn setup() -> Self {
        Self(HashMap::new())
    }
    pub fn serve(&mut self, address: &'static str) -> ServerResult {
        let listener = TcpListener::bind(address)?;
        for stream in listener.incoming() {
            let mut stream = stream?;
            let mut buffer = [0; BUF_SIZE];

            stream.read(&mut buffer)?;
            let (path, method, request) = parse_stream(&buffer)?;

            println!("requested: {:?} {}", method, path);

            let response = 'res: {
                let Some(handlers) = self.0.get(path) else {
                    break 'res Response::NotFound()
                };
                let Some(handler) = handlers.get(&method) else {
                    break 'res Response::NotFound()
                };
                handler(request)
            };
            stream.write(response.into_bytes())?;
            stream.flush()?
        }
        Ok(())
    }

    #[allow(non_snake_case)]
    pub fn GET(&mut self,
        path:    &'static str,
        handler: fn(Request) -> Response,
    ) -> &mut Self {
        assert!(path.starts_with("/"));
        self.0
            .entry(path)
            .and_modify(|map|
                assert_eq!(None,
                    map.insert(Method::GET, handler),
                "handler for `GET {}` is already resistered", path)
            ).or_insert(
                HashMap::from([(Method::GET, handler)])
            );
        self
    }
    #[allow(non_snake_case)]
    pub fn POST(&mut self,
        path:    &'static str,
        handler: fn(Request) -> Response,
    ) -> &mut Self {
        assert!(path.starts_with("/"));
        self.0
            .entry(path)
            .and_modify(|map|
                assert_eq!(None,
                    map.insert(Method::POST, handler),
                "handler for `POST {}` is already resistered", path)
            ).or_insert(
                HashMap::from([(Method::GET, handler)])
            );
        self
    }
}


fn parse_stream(buffer: &[u8; BUF_SIZE]) -> Result<(&str, Method, Request), ServerError> {
    let request_status = {
        let mut end_of_reqest_status = BUF_SIZE;
        for pos in 0..BUF_SIZE {
            if buffer[pos]   == b'\r'  
            && buffer[pos+1] == b'\n' {
                if pos == 0 {
                    return Err(ServerError::IOError(
                        std::io::Error::new(
                            std::io::ErrorKind::Other,
                            "HTTP request starts with '\\r'"
                        )
                    ))
                }
                end_of_reqest_status = pos - 1;
                break
            }
        }
        if end_of_reqest_status == BUF_SIZE {
            return Err(ServerError::IOError(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "HTTP request doesn't contain any valid request status",
                )
            ))
        }
        &buffer[..=end_of_reqest_status]
    };

    let mut split = request_status.split(|b| *b == b' ');
    let method = match split.next().expect("no method found in request") {
        b"GET"  => Method::GET,
        b"POST" => Method::POST,
        _ => return Err(ServerError::IOError(
            std::io::Error::new(
                std::io::ErrorKind::Other,
                "HTTP request doesn't contain any valid method",
            )
        ))
    };
    let path = std::str::from_utf8(
        split.next().expect("no request path found in request")
    ).expect("failed to get path from buffer");

    Ok((
        path,
        method,
        Request {
            // headers: vec![],
            body:    None,
        }
    ))
}