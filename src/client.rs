use crate::auth::Auth;

pub struct Client {
    reqw_client: reqwest::Client,
    base_url: String,
    auth: Auth,
}

impl Client {
    pub fn new(base_url: impl Into<String>, auth: Auth) -> Self {
        Self {
            auth,
            reqw_client: reqwest::Client::new(),
            base_url: base_url.into(),
        }
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    pub(crate) fn reqw_client(&self) -> &reqwest::Client {
        &self.reqw_client
    }

    pub(crate) fn auth(&self) -> &Auth {
        &self.auth
    }
}
