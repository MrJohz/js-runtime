use crate::Environment;

#[derive(Debug, PartialEq, Eq, Default)]
pub struct LiveEnvironment;

impl Environment for LiveEnvironment {
    fn path_from_base(
        &self,
        _: &std::path::Path,
        _: &std::path::Path,
    ) -> Result<std::path::PathBuf, std::io::Error> {
        todo!()
    }

    fn read_file(&self, _: &std::path::Path) -> Result<Vec<u8>, std::io::Error> {
        todo!()
    }

    fn copy_directory(
        &self,
        _: &std::path::Path,
        _: &std::path::Path,
    ) -> Result<(), std::io::Error> {
        todo!()
    }

    fn canonical(&self, _: &std::path::Path) -> Result<std::path::PathBuf, std::io::Error> {
        todo!()
    }
}
