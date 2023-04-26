pub struct Account {
    username: String,
    password: String,
    base_url: String,
}

impl Account {
    pub fn new(
        username: impl Into<String>,
        password: impl Into<String>,
        base_url: impl Into<String>,
    ) -> Self {
        Self {
            username: username.into(),
            password: password.into(),
            base_url: base_url.into(),
        }
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn password(&self) -> &str {
        &self.password
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }
}
