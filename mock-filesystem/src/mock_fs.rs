use std::{
    cell::RefCell,
    collections::HashMap,
    io::{Error, ErrorKind},
    path::{Component, Path, PathBuf},
};

use crate::{Filesystem, MFile};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileTree {
    File(String),
    Directory(HashMap<PathBuf, FileTree>),
}

#[derive(Debug, PartialEq, Eq)]
pub struct MockFilesystem {
    cwd: PathBuf,
    file_tree: RefCell<FileTree>,
}

impl MockFilesystem {
    pub fn from_tree(file_tree: HashMap<PathBuf, FileTree>) -> Self {
        Self {
            cwd: PathBuf::from("/"),
            file_tree: RefCell::new(FileTree::Directory(file_tree)),
        }
    }
    pub fn with_cwd(self, cwd: PathBuf) -> Self {
        // TODO: ensure cwd is always canonicalised
        Self {
            cwd,
            file_tree: self.file_tree,
        }
    }

    fn get_filetree(&self, path: &Path) -> Result<FileTree, Error> {
        let root = &*self.file_tree.borrow();
        let mut current = root;
        for path in [&self.cwd, path] {
            for component in path.components() {
                match component {
                    Component::Prefix(_) => todo!(),
                    Component::RootDir => current = root,
                    Component::CurDir => {}
                    Component::ParentDir => todo!(),
                    Component::Normal(path) => match current {
                        FileTree::File(_) => return Err(Error::from(ErrorKind::NotFound)),
                        FileTree::Directory(dir) => {
                            let path: &Path = path.as_ref();
                            if let Some(ft) = dir.get(path) {
                                current = ft;
                            } else {
                                return Err(Error::from(ErrorKind::NotFound));
                            }
                        }
                    },
                }
            }
        }

        Ok(current.clone())
    }
}

impl Filesystem for MockFilesystem {
    fn canonical(
        &self,
        path: impl AsRef<Path>,
        other_path: impl AsRef<Path>,
    ) -> Result<PathBuf, Error> {
        let path = path.as_ref();
        let other_path = other_path.as_ref();

        let mut created = Vec::new();
        for part in [&self.cwd, path, other_path] {
            for component in part.components() {
                match component {
                    Component::Prefix(_) => todo!(),
                    Component::RootDir => created.clear(),
                    Component::CurDir => {}
                    Component::ParentDir => {
                        created.pop().unwrap();
                    }
                    Component::Normal(str) => created.push(PathBuf::from(str)),
                }
            }
        }

        let mut new = PathBuf::from("/");
        let mut cwd = &*self.file_tree.borrow();
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

    fn load_file(&self, path: impl AsRef<Path>) -> Result<MFile, Error> {
        match dbg!(self.get_filetree(path.as_ref())?) {
            FileTree::File(text) => Ok(MFile::from(text.clone())),
            FileTree::Directory(_) => Err(Error::from(ErrorKind::NotFound)),
        }
    }

    fn copy_directory(
        &self,
        source_path: impl AsRef<Path>,
        target_path: impl AsRef<Path>,
    ) -> Result<(), Error> {
        let target_path = target_path.as_ref();
        let target_dir_path = target_path.parent().unwrap();
        let target_name = target_path.file_name().unwrap();
        let source_dir = self.get_filetree(source_path.as_ref())?;
        let target_dir = self.get_filetree(target_dir_path)?;

        let source_dir = source_dir.clone();

        match target_dir {
            FileTree::File(_) => return Err(Error::from(ErrorKind::NotFound)),
            FileTree::Directory(mut map) => map.insert(PathBuf::from(target_name), source_dir),
        };

        Ok(())
    }
}
