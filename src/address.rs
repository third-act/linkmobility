use serde::Serialize;

#[derive(Serialize)]
pub struct Address(String);

impl Address {
    pub fn new(test: String) -> Self {
        Self(test)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}
