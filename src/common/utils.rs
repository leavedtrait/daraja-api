//use surf::{get,client,Error};
use base64::{self, engine::general_purpose, Engine};
use chrono::Utc;

use crate::models::models::{AuthToken, StkResponse};
use ureq::{get, json, post, Error};

// Helper function to encode the credentials (replace with your actual encoding logic)
fn encode(data: &str) -> String {
    general_purpose::STANDARD.encode(data)
    //base64::encode(data)
}

pub async fn get_mpesa_token() -> Result<AuthToken, ureq::Error> {
    let url = "https://sandbox.safaricom.co.ke/oauth/v1/generate?grant_type=client_credentials";
    let auth_header = format!(
        "Basic {}",
        encode("oB3k4GMt4KZR9o6Msob3Qne3oDwAfxbk:XxohAhWngRijrrh3")
    );
    let auth_token: AuthToken;

    let res = get(url)
        .set("Authorization", &auth_header) // Set the Authorization header
        .call();

    match res {
        Ok(res) => {
            /* it worked */
            auth_token = res.into_json()?;
            Ok(auth_token)
        }
        Err(ureq::Error::Status(code, response)) => {
            response.status();
            /* the server returned an unexpected status
            code (such as 400, 500 etc) */
            Err(Error::Status(code, response))
        }
        Err(ureq::Error::Transport(transport)) => {
            /* some kind of io/transport error */
            Err(Error::Transport(transport))
        }
    }
}

// Assuming you have a struct `StkResponse` defined elsewhere

pub async fn init_stk(phone_num: i64, amount: i32) -> Result<StkResponse, ureq::Error> {
    let endpoint = "https://sandbox.safaricom.co.ke/mpesa/stkpush/v1/processrequest";

    let token = get_mpesa_token().await?;

    let authorization_header_value = format!("Bearer {}", token.access_token);

    let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();

    let password = format!(
        "{}{}{}",
        "174379", "bfb279f9aa9bdbcf158e97dd71a467cd2e0c893059b10f78e6b72ada1ed2c919", &timestamp
    );
    let password_base64 = encode(&password);

    let body = json!({
        "BusinessShortCode": "174379",
        "Password": password_base64,
        "Timestamp": timestamp,
        "TransactionType": "CustomerPayBillOnline",
        "PartyA": format!("{phone_num}"), // fill with your phone number
        "PartyB": "174379",
        "PhoneNumber": format!("{phone_num}"), // fill with your phone number
        "CallBackURL": "https://5938-102-0-1-146.ngrok-free.app/stkCallback",
        "AccountReference": "Johnny_Pay",
        "TransactionDesc": "HelloTest",
        "Amount": amount
    });

    let res = post(&endpoint)
        .set("Authorization", &authorization_header_value) // Set the Authorization header
        .send_json(body);

    match res {
        Ok(res) => {
            let res: StkResponse = res.into_json()?;
            Ok(res)
        }
        Err(ureq::Error::Status(code, response)) => {
            response.status();
            /* the server returned an unexpected status
            code (such as 400, 500 etc) */
            Err(Error::Status(code, response))
        }
        Err(ureq::Error::Transport(transport)) => {
            /* some kind of io/transport error */
            Err(Error::Transport(transport))
        }
    }
}
