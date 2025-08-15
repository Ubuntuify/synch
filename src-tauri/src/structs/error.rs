use std::fmt;

#[derive(Debug, Clone)]
pub struct InvalidDbError;

impl fmt::Display for InvalidDbError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "corrupted data detected in db!")
    }
}
