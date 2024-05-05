use surf::{get,client,Error};
use base64::encode;

use crate::models::models::AuthToken;

pub async fn get_mpesa_token() -> Result<AuthToken, Error> {
    let url = "https://sandbox.safaricom.co.ke/oauth/v1/generate?grant_type=client_credentials";
    let auth_header = format!(
        "Basic {}",
        encode("oB3k4GMt4KZR9o6Msob3Qne3oDwAfxbk:XxohAhWngRijrrh3")
    );

    let client = surf::client();
    let req  = get(url).header("Authorization", auth_header);
    let mut res = client.send(req).await?;
 
    


    let auth_token: AuthToken = res.body_json().await?;

    Ok(auth_token)
}