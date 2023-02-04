use std::net::TcpListener;
use actix_web::{
    web,
    HttpServer,
    App,
};
use actix_web::dev::Server;
use crate::configuration::Settings;
use crate::routes::{
    get,
    metadata,
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
) -> Result<Server, anyhow::Error> {
    let server = HttpServer::new(move || {
            App::new()
                .route("/get", web::get().to(get))
                .route("/metadata", web::get().to(metadata))
                .route("/remove", web::get().to(remove))
                .route("/store", web::post().to(store))
                .route("/exists", web::get().to(exists))
        })
        .listen(listener)?
        .run();

    Ok(server)
}
