// use chrono::ParseError;
use std::{io, path::StripPrefixError};
use tauri::ipc::InvokeError;
use thiserror::Error;

// use anyhow::{Context, Result, Error};
// use git2::Repository;
// use std::fs::File;

#[derive(Error, Debug)]
// #[derive(Debug)]
pub(crate) enum GitFrontendError {
    #[error("IO error: {0}")]
    IoError(#[from] io::Error),

    // #[error("Parse error: {0}")]
    // ParseError(#[from] chrono::ParseError),

    #[error("Git2 error: {0}")]
    GitError(#[from] git2::Error),

    #[error("Strip prefix error: {0}")]
    StripPrefixError(#[from] std::path::StripPrefixError),

    // #[error("Invalid repository path: {0}")]//TODO: Check why we have this and remove if not needed
    // InvalidPathError(String),

    // #[error("BackTrace: {0}")] //TODO: Probably can be removed
    // BackTrace(#[from] backtrace::Backtrace),

    #[error("Anyhow error: {0}")]
    AnyhowError(#[from] anyhow::Error),

    #[error("Other error: {0}")]
    OtherError(String),
}

// Implement `Into<InvokeError>` for `GitFrontendError`
impl From<GitFrontendError> for InvokeError {
    fn from(err: GitFrontendError) -> InvokeError {
        InvokeError::from(err.to_string())
    }
}
