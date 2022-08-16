use std::convert::Infallible;
use thiserror::Error;
use mobc_postgres::tokio_postgres;
use serde::Serialize;
use warp::{Rejection, Reply, hyper::StatusCode};


#[derive(Error, Debug)]
pub enum Error {
    #[error("error getting connection from DB pool: {0}")]
    DBPoolError(mobc::Error<tokio_postgres::Error>),
    #[error("error execute DB query : {0}")]
    DBQueryError(#[from] tokio_postgres::Error),
    #[error("error creating table: {0}")]
    DBInitError(tokio_postgres::Error),
    #[error("error reading file: {0}")]
    ReadFileError(#[from] std::io::Error),
}

#[derive(Serialize)]
struct ErrorResponse {
    message: String,
}

impl warp::reject::Reject for Error {}


pub async fn handle_rejection(err: Rejection) -> std::result::Result<impl Reply, Infallible> {
    let status;
    let message;

    if err.is_not_found() {
        status = StatusCode::NOT_FOUND;
        message = "Not Found";
    } else if let Some(body_err) = err.find::<warp::filters::body::BodyDeserializeError>() {
        eprintln!("invalid body: {}", body_err);
        status = StatusCode::BAD_REQUEST;
        message = "Invalid Body";
    } else if let Some(e) = err.find::<Error>() {
        match e {
            Error::DBQueryError(_) => {
                status = StatusCode::INTERNAL_SERVER_ERROR;
                message = "Could not execute request";
            },
            _ => {
                eprintln!("Unhandled application error: {:?}", err);
                status = StatusCode::INTERNAL_SERVER_ERROR;
                message = "Internal server error";
            }
        }
    } else if err.find::<warp::reject::MethodNotAllowed>().is_some() {
        status = StatusCode::METHOD_NOT_ALLOWED;
        message = "Method not Allowed";
    } else {
        eprintln!("unhandled error {:?}", err);
        status = StatusCode::INTERNAL_SERVER_ERROR;
        message = "Internal server error";
    }

    let json = warp::reply::json(&ErrorResponse {
        message: message.into(),
    });

    Ok(warp::reply::with_status(json, status))
}