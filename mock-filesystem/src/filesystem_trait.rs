use std::{
    fs::File,
    io::{Cursor, Error, Read},
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub enum MFile {
    RealFile(File),
    MockFile(Cursor<Vec<u8>>),
}

impl PartialEq for MFile {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::MockFile(l0), Self::MockFile(r0)) => l0 == r0,
            (_, _) => false,
        }
    }
}

impl Eq for MFile {}

impl From<String> for MFile {
    fn from(file_contents: String) -> Self {
        Self::MockFile(Cursor::new(file_contents.into()))
    }
}

impl From<&str> for MFile {
    fn from(file_contents: &str) -> Self {
        Self::MockFile(Cursor::new(file_contents.into()))
    }
}

impl Read for MFile {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match self {
            MFile::RealFile(f) => f.read(buf),
            MFile::MockFile(v) => v.read(buf),
        }
    }
}

pub trait Filesystem {
    fn canonical(
        &self,
        path: impl AsRef<Path>,
        other_path: impl AsRef<Path>,
    ) -> Result<PathBuf, Error>;

    fn load_file(&self, path: impl AsRef<Path>) -> Result<MFile, Error>;
}
