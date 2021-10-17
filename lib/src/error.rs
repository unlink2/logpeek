#[derive(Debug)]
pub enum Error {
    RegexError(regex::Error),
}

impl From<regex::Error> for Error {
    fn from(error: regex::Error) -> Self {
        Error::RegexError(error)
    }
}
