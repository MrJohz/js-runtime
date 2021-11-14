use std::path::PathBuf;

use mock_filesystem::{mock_tree, Filesystem};

#[test]
fn resolves_canonical_directory_from_joined_relative_paths() {
    let fs = mock_tree!(
        "path" => {
            "to" => {
                "a" => {}
            }
        }
    );

    assert_eq!(
        fs.canonical("/path", "./to/a").unwrap(),
        PathBuf::from("/path/to/a")
    );
}

#[test]
fn resolves_canonical_file_from_joined_relative_paths() {
    let fs = mock_tree!(
        "path" => {
            "to" => {
                "a.txt" => "this is a file"
            }
        }
    );

    assert_eq!(
        fs.canonical("/path", "./to/a.txt").unwrap(),
        PathBuf::from("/path/to/a.txt")
    );
}

#[test]
fn resolves_canonical_file_from_joined_absolute_path() {
    let fs = mock_tree!(
        "path-1" => {
            "to" => {
                "a.txt" => "this is a file"
            }
        }
        "path-2" => {
            "to" => {
                "a.txt" => "this is a file"
            }
        }
    );

    assert_eq!(
        fs.canonical("/path-1", "/path-2/to/a.txt").unwrap(),
        PathBuf::from("/path-2/to/a.txt")
    );
}

#[test]
fn resolves_canonical_file_with_parent_dir_components() {
    let fs = mock_tree!(
        "path-1" => {
            "to" => {
                "a.txt" => "this is a file"
            }
        }
        "path-2" => {
            "to" => {
                "a.txt" => "this is a file"
            }
        }
    );

    assert_eq!(
        fs.canonical("/path-1/to", "../../path-2/to/a.txt").unwrap(),
        PathBuf::from("/path-2/to/a.txt")
    );
}
