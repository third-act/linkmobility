use crate::account::Account;

pub struct Client {
    reqw_client: reqwest::Client,
    account: Account,
}

impl Client {
    pub fn new(account: Account) -> Self {
        Self {
            account,
            reqw_client: reqwest::Client::new(),
        }
    }

    pub(crate) fn reqw_client(&self) -> &reqwest::Client {
        &self.reqw_client
    }

    pub(crate) fn account(&self) -> &Account {
        &self.account
    }
}
