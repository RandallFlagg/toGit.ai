use chrono::ParseError;
// use tauri::ipc::InvokeError;
use std::{fmt, io};
use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum GitFrontendError {
    #[error("IO error")]
    Io(#[from] io::Error),
    #[error("Parse error")]
    Parse(#[from] chrono::ParseError),
    #[error("Git2 error")]
    Git2(#[from] git2::Error),
    #[error("Invalid repository path")]
    InvalidPath(String),
    #[error("Other error")]
    Other(String),
}

// impl fmt::Display for GitFrontendError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             GitFrontendError::Io(err) => write!(f, "IO error: {}", err),
//             GitFrontendError::Parse(err) => write!(f, "Parse error: {}", err),
//             GitFrontendError::Git2(err) => write!(f, "Git2 error: {}", err),
//             GitFrontendError::InvalidPath(msg) => write!(f, "Invalid path: {}", msg),
//             GitFrontendError::Other(msg) => write!(f, "Other error: {}", msg),
//         }
//     }
// }

// impl Into<String> for GitFrontendError {
//     fn into(self) -> String {
//         //InvokeError::from(self.to_string())
//         // Ok("Error")
//         "Error".to_string()
//     }
// }

impl From<String> for GitFrontendError {
    fn from(err: String) -> GitFrontendError {
        GitFrontendError::Other(err)
    }
}

// impl fmt::Display for FileError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             FileError::Io(err) => write!(f, "I/O Error: {}", err),
//             FileError::Parse(err) => write!(f, "Parse Error: {}", err),
//             FileError::Other(err) => write!(f, "Other Error: {}", err),
//         }
//     }
// }

// impl From<io::Error> for FileError {
//     fn from(err: io::Error) -> FileError {
//         FileError::Io(err)
//     }
// }

// impl From<ParseError> for FileError {
//     fn from(err: ParseError) -> FileError {
//         FileError::Parse(err)
//     }
// }
