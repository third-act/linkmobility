use linkmobility::address::Address;
use linkmobility::auth::Auth;
use linkmobility::client::Client;
use linkmobility::rest_api::{PlatformID, PlatformPartnerID, SMS};

#[tokio::main]
async fn main() {
    static USERNAME: &str = "[USERNAME]";
    static PASSWORD: &str = "[PASSWORD]";
    static SOURCE: &str = "[SOURCE PHONE NUMBER]";
    static DESTINATION: &str = "[DESTINATION PHONE NUMBER (MSISDNS)]";
    static PLATFORM_ID: &str = "COMMON_API";
    static PLATFORM_PARTNER_ID: &str = "[PLATFORM PARTNER ID]";

    let client = Client::new("[BASE URL]", Auth::new(USERNAME, PASSWORD));
    let response = SMS::with_text(
        Address::new(SOURCE.to_string()),
        Address::new(DESTINATION.to_string()),
        PlatformID::new(PLATFORM_ID),
        PlatformPartnerID::new(PLATFORM_PARTNER_ID),
        "Sms via Link Mobility",
    )
    .send(&client)
    .await
    .expect("error");
    println!("{response:?}");
}
