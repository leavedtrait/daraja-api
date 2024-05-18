use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthToken {
    pub access_token: String,
    pub expires_in: String,
}

#[allow(non_snake_case)] // Allow non-snake-case identifiers for this struct
#[derive(Serialize, Deserialize, Debug)]
pub struct StkPush {
    pub BusinessShortCode: String,
    pub Password: String,
    pub Timestamp: String,
    pub TransactionType: String,
    pub Amount: String,
    pub PartyA: String,
    pub PartyB: String,
    pub PhoneNumber: String,
    pub CallBackURL: String,
    pub AccountReference: String,
    pub TransactionDesc: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct StkResponse {
    pub MerchantRequestID: String,
    pub CheckoutRequestID: String,
    pub ResponseCode: String,
    pub ResponseDescription: String,
    pub CustomerMessage: String,
}

#[derive(Debug, Deserialize)]
pub struct StkQueryParams {
    pub phone_number: i64,
    pub amount: i32,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct CallbackMetadataItem {
    pub Name: String,
    pub Value: Value, // Use serde_json::Value for flexible value types
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct CallbackMetadata {
    pub Item: Vec<CallbackMetadataItem>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct StkCallback {
    pub MerchantRequestID: String,
    pub CheckoutRequestID: String,
    pub ResultCode: u32,
    pub ResultDesc: String,
    pub CallbackMetadata: CallbackMetadata,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Body {
    pub stkCallback: StkCallback,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct StkFinalResponse {
    pub Body: Body,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct UnifiedResponse {
    // Define a single unified response structure
    pub MerchantRequestID: Option<String>,
    pub CheckoutRequestID: Option<String>,
    pub ResponseCode: Option<String>,
    pub ResponseDescription: Option<String>,
    pub CustomerMessage: Option<String>,
    pub Body: Option<Body>,
}
