use std::{
    collections::HashMap,
    io::{Error, ErrorKind},
    path::PathBuf,
};

use super::Filesystem;

#[derive(Debug)]
pub enum FileTree {
    File(String),
    Directory(HashMap<PathBuf, FileTree>),
}

#[derive(Debug)]
pub struct MockFilesystem {
    cwd: PathBuf,
    file_tree: HashMap<PathBuf, FileTree>,
}

impl MockFilesystem {
    pub fn from_tree(file_tree: HashMap<PathBuf, FileTree>) -> Self {
        Self {
            cwd: PathBuf::from("/"),
            file_tree: file_tree,
        }
    }
    pub fn with_cwd(self, cwd: PathBuf) -> Self {
        // TODO: ensure cwd is always canonicalised
        Self {
            cwd,
            file_tree: self.file_tree,
        }
    }
}

impl Filesystem for MockFilesystem {
    fn canonical(
        &self,
        path: impl AsRef<std::path::Path>,
        other_path: impl AsRef<std::path::Path>,
    ) -> Result<std::path::PathBuf, Error> {
        let path = path.as_ref();
        let other_path = other_path.as_ref();

        let mut created = Vec::new();
        for part in [&self.cwd, path, other_path] {
            for component in part.components() {
                match component {
                    std::path::Component::Prefix(_) => todo!(),
                    std::path::Component::RootDir => created.clear(),
                    std::path::Component::CurDir => {}
                    std::path::Component::ParentDir => {
                        created.pop();
                    }
                    std::path::Component::Normal(str) => created.push(PathBuf::from(str)),
                }
            }
        }

        let mut new = PathBuf::from("/");
        let mut cwd = Some(&self.file_tree);
        for cpt in created {
            if let Some(directory) = cwd {
                if let Some(ft) = directory.get(&cpt) {
                    new = new.join(cpt);
                    cwd = match ft {
                        FileTree::File(_) => None,
                        FileTree::Directory(hm) => Some(hm),
                    }
                } else {
                    return Err(Error::from(ErrorKind::NotFound));
                }
            } else {
                return Err(Error::from(ErrorKind::NotFound));
            }
        }

        Ok(new)
    }

    fn load_manifest(
        &self,
        path: impl AsRef<std::path::Path>,
    ) -> Result<manifests::PackageConfig, Error> {
        todo!()
    }
}

#[macro_export]
macro_rules! mock_fs {
    (
        $($path:literal => $child:tt)*
    ) => {{
        let mut hm = ::std::collections::HashMap::new();
        $(
            hm.insert(
                ::std::path::PathBuf::from($path),
                ::mock_filesystem::mock_fs! { @child $child },
            );
        )*

        ::mock_filesystem::MockFilesystem::from_tree(hm)
    }};
    (@child {
        $($path:literal => $child:tt)*
    }) => {{
        let mut hm = ::std::collections::HashMap::new();
        $(
            hm.insert(
                ::std::path::PathBuf::from($path),
                ::mock_filesystem::mock_fs! { @child $child },
            );
        )*

        ::mock_filesystem::mock_filesystem::FileTree::Directory(hm)
    }};
    (@child $child:literal) => {
        ::mock_filesystem::mock_filesystem::FileTree::File(String::from($child))
    };
}
