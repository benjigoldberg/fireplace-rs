use crate::server;
use actix_web::error;
use actix_web::{web, HttpResponse, Responder};
use serde_derive::Deserialize;
use std::sync::{Arc, Mutex};

#[derive(Deserialize)]
pub struct FireplaceFormData {
    status: String,
}

pub async fn state_handler(
    form: web::Form<FireplaceFormData>,
    data: web::Data<Arc<Mutex<server::Data>>>,
) -> impl Responder {
    let (fan, flame) = match form.status.as_str() {
        "allOn" => (true, true),
        "fireplaceOn" => (false, true),
        "allOff" => (false, false),
        _ => {
            return Err(error::ErrorBadRequest(format!(
                "unknown status: {}",
                form.status
            )))
        }
    };
    let mut fireplace_state = data.lock().unwrap();
    match fireplace_state.fireplace.set(fan, flame) {
        Ok(_) => Ok(HttpResponse::Ok()),
        Err(e) => Err(error::ErrorBadRequest(format!(
            "Unable to set fireplace state: {}",
            e
        ))),
    }
}
