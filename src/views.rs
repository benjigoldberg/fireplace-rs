use crate::server;
use actix_web::error;
use actix_web::{web, HttpResponse, Responder};
use serde_derive::Deserialize;
use std::sync::{Arc, Mutex};
use std::io::{Error, ErrorKind};

#[derive(Deserialize)]
pub struct FireplaceFormData {
    status: String,
}

pub enum Status {
    AllOn,
    FireplaceOn,
    AllOff,
}

impl Status {
    fn new(status: &str) -> Result<Status, std::io::Error> {
        match status {
            "allOn" => Ok(Status::AllOn),
            "fireplaceOn" => Ok(Status::FireplaceOn),
            "allOff" => Ok(Status::AllOff),
            _ => Err(Error::new(ErrorKind::InvalidData, format!("invalid status: {}", status)))
        }
    }

    fn to_state(&self) -> (bool, bool) {
        match self {
            Status::AllOn => (true, true),
            Status::FireplaceOn => (false, true),
            Status::AllOff => (false, false),
        }
    }
}

pub async fn state_handler(
    form: web::Form<FireplaceFormData>,
    data: web::Data<Arc<Mutex<server::Data>>>,
) -> impl Responder {
    Status::new(form.status.as_str())
        .map_err(|e| error::ErrorBadRequest(format!("unable to process request: {}", e)))
        .map(|status| {
            data.lock()
                .map_err(|_| error::ErrorInternalServerError("Internal Server Error"))
                .and_then(|mut fireplace_state| {
                    fireplace_state.fireplace.set(status.to_state())
                        .map_err(|e| {
                            error::ErrorBadRequest(format!("Unable to set fireplace state: {}", e))
                        })
                        .map(|_| HttpResponse::Ok())
                })
        })
}
