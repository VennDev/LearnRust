#[derive(Debug)]
pub enum ProviderKind {
    Name(String),
    Email(String),
    Address(String),
}

#[derive(Debug)]
pub struct _Provider {
    pub kind: ProviderKind,
    pub info: String,
}
