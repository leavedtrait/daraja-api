// handlers go here
use  crate::common::utils;
use tide::Request;
use tide::prelude::*;

pub async fn get_token(mut req: Request<()>) -> tide::Result {
    let res = utils::get_mpesa_token().await?;

    let response = serde_json::to_string(&res)?;

    Ok(response.into())
}