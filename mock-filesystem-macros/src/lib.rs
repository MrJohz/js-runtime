#[macro_export]
macro_rules! mock_tree {
    (
        $($path:literal => $child:tt)*
    ) => {{
        let mut hm = ::std::collections::HashMap::new();
        $(
            hm.insert(
                ::std::path::PathBuf::from($path),
                ::mock_filesystem::mock_tree! { @child $child },
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
                ::mock_filesystem::mock_tree! { @child $child },
            );
        )*

        ::mock_filesystem::mock_fs::FileTree::Directory(hm)
    }};
    (@child $child:expr) => {
        ::mock_filesystem::mock_fs::FileTree::File(String::from($child))
    };
}
