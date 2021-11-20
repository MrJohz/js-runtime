use crate::Environment;

pub struct NullEnvironment;

impl Environment for NullEnvironment {
    fn path_from_base(
        &self,
        _: &std::path::Path,
        _: &std::path::Path,
    ) -> Result<std::path::PathBuf, std::io::Error> {
        panic!("The null environment cannot be used");
    }

    fn load_file(&self, _: &std::path::Path) -> Result<std::fs::File, std::io::Error> {
        panic!("The null environment cannot be used");
    }

    fn copy_directory(
        &self,
        _: &std::path::Path,
        _: &std::path::Path,
    ) -> Result<(), std::io::Error> {
        panic!("The null environment cannot be used");
    }
}
