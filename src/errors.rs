use std::fmt;

// #[derive(Debug)]
// pub enum Error {
//     UnableToParse,
//     FileNotFound,
//     FileRead,
//     CreateFile,
//     Write,
//     Append,
//     DataNotFound,
//     IncorrectFileType,
// }


#[derive(Debug, Clone)]
pub struct IncorrectFileType {
    pub message: String,
}

impl fmt::Display for IncorrectFileType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Incorrect file type. You must use the \".CSV\" file extension.")
    }
}

impl std::error::Error for IncorrectFileType {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
    fn description(&self) -> &str {
        &self.message
    }
    fn cause(&self) -> Option<&dyn std::error::Error> {
        None
    }
}