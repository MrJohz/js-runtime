/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

#[derive(Debug)]
pub enum ResolveFailure {
    IoError(std::io::Error),
    ManifestParseError(manifests::Error),
}

impl From<std::io::Error> for ResolveFailure {
    fn from(error: std::io::Error) -> Self {
        Self::IoError(error)
    }
}

impl From<manifests::Error> for ResolveFailure {
    fn from(error: manifests::Error) -> Self {
        Self::ManifestParseError(error)
    }
}
