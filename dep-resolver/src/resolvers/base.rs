use super::FileResolver;

pub trait Resolver {
    fn as_file_resolver<'a>(&'a self) -> Option<&'a FileResolver<'a>> {
        None
    }
}
