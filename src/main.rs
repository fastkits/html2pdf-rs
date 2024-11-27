
mod routers;
mod html2pdf;
mod vo;
mod infra;

use tokio;
use actix_web::{web, App, HttpServer};
use actix_web::middleware::Logger;
use env_logger::Env;


#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("debug"));
    let server = HttpServer::new(move || {
        let mut app = App::new();
        app = app.service(
            web::scope("")
                .configure(routers::register_advert_routes)
        );
        app.wrap(Logger::new(
            r#"%a "%r" %s %b "%{Content-Length}i" "%{Referer}i" "%{User-Agent}i" %T"#,
        ))
    })
        // .bind((cfg.http.addr.as_str(), cfg.http.port))?;
        .bind(("0.0.0.0", 8080))?;

    let server = server
        .workers(4)
        .disable_signals()
        .run();
    server.await?;
    Ok(())
}
