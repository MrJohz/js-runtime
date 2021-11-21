use std::{
    fs::{canonicalize, File},
    io::Read,
};

use crate::Environment;

#[derive(Debug, PartialEq, Eq, Default)]
pub struct LiveEnvironment;

impl Environment for LiveEnvironment {
    fn path_from_base(
        &self,
        base: &std::path::Path,
        path: &std::path::Path,
    ) -> Result<std::path::PathBuf, std::io::Error> {
        self.canonical(&base.join(path))
    }

    fn read_file(&self, path: &std::path::Path) -> Result<Vec<u8>, std::io::Error> {
        let mut file = File::open(path)?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;
        Ok(buf)
    }

    fn copy_directory(
        &self,
        _: &std::path::Path,
        _: &std::path::Path,
    ) -> Result<(), std::io::Error> {
        todo!("copy_directory")
    }

    fn canonical(&self, path: &std::path::Path) -> Result<std::path::PathBuf, std::io::Error> {
        canonicalize(path)
    }
}
