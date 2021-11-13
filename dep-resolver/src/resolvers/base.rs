pub trait Resolver {
    #[cfg(test)]
    fn as_file_resolver<'a>(&'a self) -> Option<&'a super::FileResolver<'a>> {
        None
    }
}
