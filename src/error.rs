/// Used for any parsing error for any supported format.
#[derive(Debug)]
pub struct ParseError {
    pub detail: String,
}

impl ParseError {
    pub fn new(s: &str) -> ParseError {
        ParseError{
            detail: String::from(s),
        }
    }
}
