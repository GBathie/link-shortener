use std::fmt::Display;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct ShortLinkId(String);

impl ShortLinkId {
    pub fn new(id: String) -> Self {
        Self(id)
    }

    pub fn id(&self) -> &str {
        &self.0
    }

    pub fn into_id(self) -> String {
        self.0
    }
}

impl Display for ShortLinkId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
