/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::Environment;

#[derive(Debug)]
pub struct NullEnvironment;

impl Environment for NullEnvironment {
    fn path_from_base(
        &self,
        _: &std::path::Path,
        _: &std::path::Path,
    ) -> Result<std::path::PathBuf, std::io::Error> {
        panic!("The null environment cannot be used");
    }

    fn read_file(&self, _: &std::path::Path) -> Result<Vec<u8>, std::io::Error> {
        panic!("The null environment cannot be used");
    }

    fn copy_directory(
        &self,
        _: &std::path::Path,
        _: &std::path::Path,
    ) -> Result<(), std::io::Error> {
        panic!("The null environment cannot be used");
    }

    fn canonical(&self, _: &std::path::Path) -> Result<std::path::PathBuf, std::io::Error> {
        panic!("The null environment cannot be used");
    }
}
