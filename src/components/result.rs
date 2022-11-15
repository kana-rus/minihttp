pub type ServerResult = Result<(), ServerError>;
#[derive(Debug)]
pub enum ServerError {
    IOError(std::io::Error)
}
impl From<std::io::Error> for ServerError {
    fn from(value: std::io::Error) -> Self {
        Self::IOError(value)
    }
}