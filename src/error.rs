use std::string::ParseError;

#[derive(Clone, Copy, Debug)]
pub enum ErrorCode {
    /// Invalid authentication. Please check your username and password.
    InvalidAuth,
    /// Access denied. Please check your username and password.
    AccessDenied,
    /// Unknown Error. Please contact Support and include your whole transaction.
    Unknown,
    /// Parse Error. The object to send in the request is badly formatted.
    Parse,
    /// Unable to access SMSC credentials.
    SMSCCredentials,
    /// Invalid or missing platformId. Please check your platformId.
    InvOrMissingPlatformID,
    /// Invalid or missing platformPartnerId. Please check your platformPartnerId.
    InvOrMissingPlatformPartnerID,
    /// Invalid or missing currency for premium message. Please check your price
    /// and currency.
    InvOrMissingCurrencyForPremium,
    /// No gates available. Please contact Support and include your whole
    /// transaction.
    NoGatesAvail,
    /// Specified gate unavailable. Please check your gateId.
    SpecifiedGateUnavail,
    /// Linkmobility-unrelated error.
    Other,
}

#[derive(Debug)]
pub struct Error {
    code: ErrorCode,
    msg: Option<String>,
}

pub type Result<T> = core::result::Result<T, Error>;

impl Error {
    pub fn new(code: ErrorCode, msg: Option<String>) -> Self {
        Self { code, msg }
    }

    pub fn with_code(code: ErrorCode) -> Self {
        Self::new(code, None)
    }

    pub fn code(&self) -> ErrorCode {
        self.code
    }

    pub fn msg(&self) -> Option<&String> {
        self.msg.as_ref()
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "{}{}",
            match &self.code {
                ErrorCode::InvalidAuth => "invalid auth",
                ErrorCode::AccessDenied => "access denied",
                ErrorCode::Unknown => "unknown",
                ErrorCode::Parse => "parse",
                ErrorCode::SMSCCredentials => "sms credentials",
                ErrorCode::InvOrMissingPlatformID => "invalid or missing platform",
                ErrorCode::InvOrMissingPlatformPartnerID =>
                    "invalid or missing platform partner id",
                ErrorCode::InvOrMissingCurrencyForPremium =>
                    "invalid or missing currenct for premium",
                ErrorCode::NoGatesAvail => "no gates available",
                ErrorCode::SpecifiedGateUnavail => "specified gate unavailable",
                ErrorCode::Other => "other",
            },
            if let Some(msg) = &self.msg {
                format!(": {msg}")
            } else {
                "".to_string()
            }
        ))
    }
}

impl std::error::Error for Error {}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Self::new(ErrorCode::Other, Some(value.to_string()))
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::new(ErrorCode::Parse, Some(value.to_string()))
    }
}

impl From<ParseError> for Error {
    fn from(value: ParseError) -> Self {
        Self::new(ErrorCode::Other, Some(value.to_string()))
    }
}
