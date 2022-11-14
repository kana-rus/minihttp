use std::{collections::HashMap, net::TcpListener, io::{Write, Read}};
use crate::components::{Method, Request, Response, BUF_SIZE, parse_stream, ServerResult};


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

            println!("requested: {:?} {}", method,  path);

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