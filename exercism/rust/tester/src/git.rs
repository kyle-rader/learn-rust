use std::{path::{PathBuf}, process::Command};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum TestingError {
    #[error("Could not find the git root 🤔: {0}")]
    NoGitRoot(String),

    #[error("Process Io err: {0}")]
    Io(#[from] std::io::Error),

    #[error("Encoding err: {0}")]
    Encoding(#[from] std::str::Utf8Error)
}

pub fn git_root() -> Result<PathBuf, TestingError> {
    let output = Command::new("git")
        .arg("rev-parse")
        .arg("--show-toplevel")
        .output()?;

    if output.status.success() {
        Ok(std::str::from_utf8(&output.stdout)?.trim().into())
    }
    else {
        Err(TestingError::NoGitRoot(std::str::from_utf8(&output.stderr)?.into()))
    }
}

#[cfg(test)]
mod git_tests {
    use super::git_root;

    #[test]
    fn test_git_root() {
        let subject = git_root();
        let mut expected = std::env::current_dir().expect("we should be able to get the cwd");
        expected.pop(); // git-root/exercism/rust/
        expected.pop(); // git-root/exercism/
        expected.pop(); // git-root/
        assert_eq!(subject.unwrap(), expected);
    }
}