use std::{
    collections::HashMap,
    io::{Error, ErrorKind},
    path::{Path, PathBuf},
};

use crate::{Filesystem, MFile};

#[derive(Debug)]
pub enum FileTree {
    File(String),
    Directory(HashMap<PathBuf, FileTree>),
}

#[derive(Debug)]
pub struct MockFilesystem {
    cwd: PathBuf,
    file_tree: FileTree,
}

impl MockFilesystem {
    pub fn from_tree(file_tree: HashMap<PathBuf, FileTree>) -> Self {
        Self {
            cwd: PathBuf::from("/"),
            file_tree: FileTree::Directory(file_tree),
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
        let mut cwd = &self.file_tree;
        for cpt in created {
            match cwd {
                FileTree::File(_) => return Err(Error::from(ErrorKind::NotFound)),
                FileTree::Directory(dir) => {
                    if let Some(ft) = dir.get(&cpt) {
                        new = new.join(cpt);
                        cwd = ft;
                    } else {
                        return Err(Error::from(ErrorKind::NotFound));
                    }
                }
            }
        }

        Ok(new)
    }

    fn load_file(&self, path: impl AsRef<std::path::Path>) -> Result<MFile, Error> {
        let mut root = &self.file_tree;
        for path in [&self.cwd, path.as_ref()] {
            for component in path.components() {
                match component {
                    std::path::Component::Prefix(_) => todo!(),
                    std::path::Component::RootDir => root = &self.file_tree,
                    std::path::Component::CurDir => {}
                    std::path::Component::ParentDir => todo!(),
                    std::path::Component::Normal(path) => match root {
                        FileTree::File(_) => return Err(Error::from(ErrorKind::NotFound)),
                        FileTree::Directory(dir) => {
                            let path: &Path = path.as_ref();
                            if let Some(ft) = dbg!(dir.get(path)) {
                                root = ft;
                            } else {
                                return Err(Error::from(ErrorKind::NotFound));
                            }
                        }
                    },
                }
            }
        }

        dbg!(root);

        match root {
            FileTree::File(text) => Ok(MFile::from(text.clone())),
            FileTree::Directory(_) => Err(Error::from(ErrorKind::NotFound)),
        }
    }
}
