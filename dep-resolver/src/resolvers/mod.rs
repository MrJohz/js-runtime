mod base_resolver;
mod file_resolver;
mod git_resolver;

pub use base_resolver::Resolver;
pub use file_resolver::FileResolver;
pub use git_resolver::GitResolver;
