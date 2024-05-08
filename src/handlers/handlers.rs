// handlers go here
use crate::common::utils;
use tide::Request;

pub async fn get_token(mut _req: Request<()>) -> tide::Result {
    let res = utils::get_mpesa_token().await;
    match res {
        Ok(res) => {
            let response = serde_json::to_string(&res)?;

            Ok(response.into())
        }
        Err(err) => {
            let res = err.to_string();
            let err = tide::Error::from_str(tide::StatusCode::FailedDependency, res);
            Err(err)
        }
    }
}
