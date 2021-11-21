/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::{
    fs::{canonicalize, File},
    io::Read,
};

use dircpy::copy_dir;

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
        source: &std::path::Path,
        dest: &std::path::Path,
    ) -> Result<(), std::io::Error> {
        copy_dir(source, dest)
    }

    fn canonical(&self, path: &std::path::Path) -> Result<std::path::PathBuf, std::io::Error> {
        canonicalize(path)
    }
}
