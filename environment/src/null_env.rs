use crate::Environment;

pub struct NullEnvironment;

impl Environment for NullEnvironment {
    fn path_from_base(
        &self,
        _: impl AsRef<std::path::Path>,
        _: impl AsRef<std::path::Path>,
    ) -> Result<std::path::PathBuf, std::io::Error> {
        panic!("The null environment cannot be used");
    }

    fn load_file(&self, _: impl AsRef<std::path::Path>) -> Result<std::fs::File, std::io::Error> {
        panic!("The null environment cannot be used");
    }

    fn copy_directory(
        &self,
        _: impl AsRef<std::path::Path>,
        _: impl AsRef<std::path::Path>,
    ) -> Result<(), std::io::Error> {
        panic!("The null environment cannot be used");
    }
}
