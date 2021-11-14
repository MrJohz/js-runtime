use mock_filesystem::{mock_tree, Filesystem, MFile};

#[test]
fn opens_file_with_absolute_dir() {
    let fs = mock_tree!(
        "a" => { "b.txt" => "contents" }
    );

    assert_eq!(fs.load_file("/a/b.txt").unwrap(), MFile::from("contents"));
}
