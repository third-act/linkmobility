use crate::client::Client;
use crate::error::{Error, ErrorCode, Result};
use reqwest::Url;

pub(crate) struct RequestBuilder<'a> {
    inner: reqwest::RequestBuilder,
    client: &'a Client,
}

#[derive(Debug)]
pub struct Response(reqwest::Response);

impl Response {
    pub(crate) fn new(response: reqwest::Response) -> Self {
        Self(response)
    }
}

impl<'a> RequestBuilder<'a> {
    pub fn post(client: &'a Client, url: &str) -> Result<Self> {
        let url = Self::create_url(client, url)?;
        let inner = client.reqw_client().post(url);
        Ok(Self { client, inner })
    }

    pub fn body(mut self, val: impl Into<reqwest::Body>) -> Self {
        self.inner = self.inner.body(val);
        self
    }

    pub async fn execute(self) -> Result<Response> {
        let auth = self.client.auth();
        let request = self
            .inner
            .basic_auth(auth.username(), Some(auth.password()))
            .header("Content-Type", "application/json")
            .build()?;
        println!("request url:      {}", request.url());
        println!("request headers:  {:?}", request.headers());
        println!(
            "request body:     {}",
            if let Some(body) = request.body() {
                String::from_utf8(body.as_bytes().unwrap().to_vec()).unwrap()
            } else {
                "None".to_string()
            }
        );
        println!("request method:   {}", request.method());
        let response = self.client.reqw_client().execute(request).await;
        match response {
            Ok(response) => {
                if response.status().is_success() {
                    Ok(Response::new(response))
                } else {
                    Err(Error::new(
                        ErrorCode::Parse,
                        Some(response.text().await.unwrap_or("no text body".to_string())),
                    ))
                }
            }
            Err(error)
                if error.is_status()
                    && error.status().expect("expected status error")
                        == reqwest::StatusCode::UNAUTHORIZED =>
            {
                Err(Error::new(ErrorCode::AccessDenied, None))
            }
            Err(error) => Err(Error::new(ErrorCode::Other, Some(error.to_string()))),
        }
    }

    fn create_url(client: &Client, path: &str) -> Result<reqwest::Url> {
        let mut url = match Url::parse(client.base_url()) {
            Ok(base) => base,
            Err(err) => {
                return Err(Error::new(
                    ErrorCode::Parse,
                    Some(format!("unable to parse base base url: {err}",)),
                ))
            }
        };
        url.set_path(&format!("sms/{path}"));
        Ok(url)
    }
}
