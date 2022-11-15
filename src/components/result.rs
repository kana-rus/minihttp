pub type ServerResult = Result<(), ServerError>;

#[derive(Debug)]
pub enum ServerError {
    IOerror(String),
    JSONerror(String),
}
impl From<std::io::Error> for ServerError {
    fn from(value: std::io::Error) -> Self {
        Self::IOerror(value.to_string())
    }
}
impl From<serde_json::Error> for ServerError {
    fn from(value: serde_json::Error) -> Self {
        Self::JSONerror(value.to_string())
    }
}