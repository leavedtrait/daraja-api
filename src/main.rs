use tide::Request;
use tide::prelude::*;
use serde::Deserialize;


use surf::{get,client,Error};
use base64::encode;

#[derive(Debug, Deserialize,Serialize)]
struct AuthToken {
    pub access_token: String,
    pub expires_in: String,
}

async fn get_mpesa_token() -> Result<AuthToken, Error> {
    let url = "https://sandbox.safaricom.co.ke/oauth/v1/generate?grant_type=client_credentials";
    let auth_header = format!(
        "Basic {}",
        encode("oB3k4GMt4KZR9o6Msob3Qne3oDwAfxbk:XxohAhWngRijrrh3")
    );

    let client = surf::client();
    let req = get(url).header("Authorization", auth_header);
    let mut res = client.send(req).await?;
 
    


    let auth_token: AuthToken = res.body_json().await?;

    Ok(auth_token)
}


#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();
    app.at("/token").get(get_token);
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}

// handlers down here

async fn get_token(mut req: Request<()>) -> tide::Result{
    let res = get_mpesa_token().await?;
    let response = serde_json::to_string(&res)?;

    Ok(response.into())

}