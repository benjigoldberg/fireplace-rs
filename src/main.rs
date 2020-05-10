use actix_web::{web, App, HttpResponse, HttpServer};
use listenfd::ListenFd;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let addr = "127.0.0.1:8000";
    let mut listenfd = ListenFd::from_env();

    let mut server = HttpServer::new(move || {
            App::new()
                .route("/", web::get().to(|| HttpResponse::Ok().body("fireplace-rs")))
        });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server.bind(addr)?
    };

    server.run().await
}
