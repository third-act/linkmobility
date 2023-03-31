use crate::address::Address;
use crate::client::Client;
use crate::error::Result;
use crate::request::{RequestBuilder, Response};
use crate::ty::{Boolean, Currency, Date, Integer, KeyValue, Long, Priority, String, ToN, DCS};
use serde::Serialize;

#[derive(Serialize)]
pub struct PlatformID(String);

impl PlatformID {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
}

#[derive(Serialize)]
pub struct PlatformPartnerID(String);

impl PlatformPartnerID {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
}

#[derive(Serialize)]
pub struct SMS {
    /// Required. This is the source number from where the
    /// message should be sent. The format is depending on
    /// the specified sourceTON.
    pub source: Address,
    /// This is the source type of number. See allowed TON
    /// values below.
    /// Default ALPHANUMERIC
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "sourceTON")]
    pub source_ton: Option<ToN>,
    /// Required. This is the destination number. The format is
    /// depending on the specified destinationTON.
    /// Remember that MSISDNS include the country code and
    /// a leading plus sign. (+)
    pub destination: Address,
    /// This is the destination type of number. See allowed
    /// TON values below.
    /// Default MSISDN.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "destinationTON")]
    pub destination_ton: Option<ToN>,
    /// Advanced.
    /// This is the Data Coding Scheme that should be used
    /// when sending the SMS. See allowed DCS values in a
    /// separate table.
    ///Default TEXT.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dcs: Option<DCS>,
    /// Advanced.
    /// This value may be specified when sending
    /// concatenated SMS, WAP-push, etc. The format is hex
    /// encoded 8-bit bytes. More information about valid
    /// UDH for long SMS may be given by Support upon
    /// request. Common will handle the splitting and
    /// concatenation of messages if you do not have a
    /// specific reason to do it yourself.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "userDataHeader")]
    pub user_data_header: Option<String>,
    //// This is the message content itself. The DCS specifies
    /// the format (encoding) on this value.
    /// Note that messages that messages of more than 140
    /// bytes must be split into multiple messages. Common
    /// will do that automatically by default.
    #[serde(rename = "userData")]
    pub user_data: String,
    /// True indicates that a delivery report should be sent
    /// back when the message has come to a final state.
    /// (Delivered or failed)
    /// TRUE is mandatory for premium messages.
    /// Defaults to TRUE, and it is recommended to use
    /// delivery reports.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "useDeliveryReport")]
    pub use_deliver_report: Option<Boolean>,
    /// One or more gates that should be used for sending
    /// delivery reports. If you do not specify any Gates to
    /// deliver Delivery Reports to, make sure to set
    /// useDeliveryReport to FALSE. See the chapter on
    /// delivery reports for more information. Required for
    /// premium messages.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "deliveryReportGates")]
    pub deliver_report_gates: Option<Box<[String]>>,
    /// This specifies how long the message is supposed to
    /// live. If the message takes longer to deliver to the
    /// handset than the validityTime, the message will be
    /// discarded.
    /// The value is specified in milliseconds.
    /// Default is 48 hours (172800000).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "relativeValidityTime")]
    pub relative_validity_time: Option<Long>,
    /// The absolute time when a message should expire.
    /// Minimum 15 minutes and maximum 48h in the future.
    /// Formatted according to RFC3339, e.g. 2010-03-30T12:59:40+02:00.
    /// Overrides relativeValidityTime if both are set.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "absoluteValidityTime")]
    pub absolute_validity_time: Option<Date>,
    /// Price, in local currency, in 1/100 of currency. For
    /// example, to send a message costing 5 NOK, this should
    /// be set to 500.
    /// If you are splitting a long message into multiple
    /// segments yourself, set price only on the first segment.
    /// Default 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tariff: Option<Integer>,
    /// The currency should be set if the default country
    /// currency not to be used. Supported currencies are
    /// NOK, SEK, DKK, EUR, LTL.
    /// Ignored for non-premium messages
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,
    /// Allowed age for (adult) content.
    /// Optional. Not supported by all operators
    #[serde(skip_serializing_if = "Option::is_none")]
    pub age: Option<Integer>,
    /// See the Priority value table, Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<Priority>,
    /// Your platformId. Provided to you by Support.
    #[serde(rename = "platformId")]
    pub platform_id: PlatformID,
    /// Your platformPartnerId. Provided to you by Support.
    #[serde(rename = "platformPartnerId")]
    pub platform_partner_id: PlatformPartnerID,
    /// Your own internal transaction ID. Not used for
    /// anything except as a reference.
    /// Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "refId")]
    pub ref_id: Option<String>,
    /// When sending premium messages, a description of the
    /// service. This will be printed on the end-user’s phone
    /// bill.
    /// Ignored for non-premium messages.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "productDescription")]
    pub product_description: Option<String>,
    /// When sending premium messages, specify which
    /// category the service is. This lets the operator know
    /// which rates to apply to the message. Support or your
    /// sales contact will help you determine the correct
    /// productCategory to set.
    /// Ignored for non-premium messages.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "productCategory")]
    pub product_category: Option<Integer>,
    /// A reference to the ID of the MO message which
    /// triggered the MT message. Required for some
    /// operators.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "moReferenceId")]
    pub mo_reference_id: Option<String>,
    /// Advanced. Additional parameters may be specified if
    /// needed.
    /// Support will advise you if you need to use custom
    /// parameters.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "customParameters")]
    pub custom_parameters: Option<KeyValue>,
    /// Indicates whether you want a response in the body
    /// when you submit the message. This is not a delivery
    /// report, only a confirmation of message submission.
    /// Default is false.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "ignoreResponse")]
    pub ignore_response: Option<Boolean>,
}

impl SMS {
    pub fn with_text(
        source: Address,
        destination: Address,
        platform_id: PlatformID,
        platform_partner_id: PlatformPartnerID,
        text: impl Into<String>,
    ) -> Self {
        Self {
            source,
            source_ton: None,
            destination,
            destination_ton: None,
            dcs: None,
            user_data_header: None,
            user_data: text.into(),
            use_deliver_report: None,
            deliver_report_gates: None,
            relative_validity_time: None,
            absolute_validity_time: None,
            tariff: None,
            currency: None,
            age: None,
            priority: None,
            platform_id,
            platform_partner_id,
            ref_id: None,
            product_description: None,
            product_category: None,
            mo_reference_id: None,
            custom_parameters: None,
            ignore_response: None,
        }
    }

    pub async fn send(&self, client: &Client) -> Result<Response> {
        Ok(RequestBuilder::post(client, "send")?
            .body(serde_json::to_string(self)?)
            .execute()
            .await?)
    }
}
