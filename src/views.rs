pub mod views {
    use actix_web::{web, HttpResponse, Responder};
    use actix_web::error;
    use serde_derive::Deserialize;
    use std::sync::Mutex;
    use crate::{server::server};

    #[derive(Deserialize)]
    pub struct FireplaceFormData {
        status: String,
    }

    pub async fn state_handler(
        form: web::Form<FireplaceFormData>,
        data: web::Data<Mutex<server::Data>>,
    ) -> impl Responder {
        let(fan, flame) = match form.status.as_str() {
            "allOn" => (true, true),
            "fireplaceOn" => (false, true),
            "allOff" => (false, false),
            _ => return Err(error::ErrorBadRequest(format!("unknown status: {}", form.status))),
        };
        let fireplace_state = data.lock().unwrap();
        let mut fireplace_controller = fireplace_state.fireplace.lock().unwrap();
        match fireplace_controller.set(fan, flame) {
            Ok(_) => Ok(HttpResponse::Ok()),
            Err(e) => Err(error::ErrorBadRequest(format!("Unable to set fireplace state: {}", e))),
        }
    }
}
