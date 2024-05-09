// handlers go here
use crate::{common::utils, models::models::{StkQueryParams, UnifiedResponse}};
use tide::{http::Response, Body, Request, StatusCode};

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


pub async fn stk_push(req: Request<()>) -> tide::Result{
    let query: StkQueryParams = req.query()?;
    let res = utils::init_stk(query.phone_number, query.amount)
        .await;

    let mut response = Response::new(StatusCode::Ok);
    match res {
        Ok(res) => {
            let res = serde_json::to_string(&res)?;
            
            response.set_body(Body::from_string(res)); // Serialize response body

            Ok(response.into())
        }
        Err(err) => {
            let res = err.to_string();
            let err = tide::Error::from_str(tide::StatusCode::FailedDependency, res);
            return Err(err)
        }
    }

    
}

pub async fn handle_stk_response(mut req: Request<()>) -> Result<Response, tide::Error> {
    let body = req.body_json::<UnifiedResponse>()
        .await?;

    if let Some(stk_response) = body.MerchantRequestID {
        println!("Received StkResponse: {:?}", stk_response);
    } else if let Some(stk_final_response) = body.Body {
        println!("Received StkFinalResponse: {:?}", stk_final_response);
    } else {
        // Handle case where neither field is present (optional)
        println!("Invalid response format");
    }

    let mut res = Response::new(StatusCode::Ok);
    res.set_body(Body::from_string(String::new())); // Empty body for acknowledgement

    Ok(res)
}
