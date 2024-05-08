//use surf::{get,client,Error};
use base64::{self, engine::general_purpose, Engine};

use crate::models::models::AuthToken;
use ureq::{get, Error};

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
