use std::net::TcpListener;
use actix_web::{
    web,
    HttpServer,
    App,
};
use actix_web::dev::Server;
use actix_cors::Cors;
use tracing_actix_web::TracingLogger;

use crate::configuration::Settings;
use crate::routes::{
    get,
    all,
    metadata,
    places,
    remove,
    store,
    exists,
};



pub struct Application {
    port: u16,
    server: Server,
}


impl Application {
    pub async fn build(
        configuration: Settings,
    ) -> Result<Self, anyhow::Error> {
        let address = format!("{}:{}", configuration.application.host, configuration.application.port);

        let listener = TcpListener::bind(address)?;
        let port = listener.local_addr().unwrap().port();
        let server = run(
            listener,
            configuration.data_path,
        ).await?;

        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    // A more expressive name that makes it clear that
    // this function only returns when the application is stopped.
    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}



pub async fn run(
    listener: TcpListener,
    data_path: String,
) -> Result<Server, anyhow::Error> {
    let server = HttpServer::new(move || {
            let cors = Cors::default()
                .allowed_origin_fn(|origin, _req_head| {
                    true
                })
                .max_age(3600);

            App::new()
                .wrap(TracingLogger::default())
                .wrap(cors)

                .route("/get", web::get().to(get))
                .route("/all", web::get().to(all))
                .route("/metadata", web::get().to(metadata))
                .route("/places", web::get().to(places))
                .route("/remove", web::post().to(remove))
                .route("/store", web::post().to(store))
                .route("/exists", web::get().to(exists))

                .app_data(web::Data::new(data_path.clone()))
        })
        .listen(listener)?
        .run();

    Ok(server)
}
