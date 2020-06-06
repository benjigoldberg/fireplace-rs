use actix_files as fs;
use actix_web::{web, middleware, App, HttpServer};
use listenfd::ListenFd;
use log::{info};
use std::sync::{Arc, Mutex};

use crate::{views, Fireplace};

pub struct Data {
    pub fireplace: Fireplace,
}

pub async fn run(address: &str, fp_state: Fireplace) -> std::io::Result<()> {
    let mut listenfd = ListenFd::from_env();
    let data = Arc::new(Mutex::new(Data {
        fireplace: fp_state,
    }));
    let mut server = HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .data(data.clone())
            .route("/fireplace", web::post().to(views::state_handler))
            .service(fs::Files::new("/", "./static/").index_file("index.html"))
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server.bind(address)?
    };

    info!("server available at {}", address);
    server.run().await
}
