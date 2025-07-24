use std::fmt::Display;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct LongUrl(String);

impl LongUrl {
    pub fn new(url: String) -> Self {
        Self(url)
    }

    pub fn url(&self) -> &str {
        &self.0
    }

    pub fn into_url(self) -> String {
        self.0
    }
}

impl Display for LongUrl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
