#[derive(Debug)]
pub enum ResolveFailure {
    InvalidPackageSpec { package_spec: String },
}
