use actix_web::{middleware::Logger, web, App, HttpServer};
use clap::Parser;
use config::Config;
use env_logger::Env;
use handler::user_handler::handle_create_initial_admin;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
mod config;
pub mod data;
pub mod error;
pub mod handler;
pub mod messages;
pub mod routes;
pub mod util;

use routes::{project_routes::register_project_routes, user_routes::register_user_routes};
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::parse();

    env_logger::init_from_env(
        Env::default()
            .filter("FOXB_LOG_LEVEL")
            .default_filter_or(if config.debug {"debug"} else {"info"}), // TODO separate config value for loglevel. debug if debug otherwise grap value, info by default
    );
    log::info!("This is an example message.");

    let pool = PgPoolOptions::new()
        .max_connections(20)
        .connect(&format!(
            "postgres://{}:{}@{}/{}",
            &config.db_user, &config.db_password, &config.db_host, &config.db_db
        ))
        .await
        .expect("Pool Failed");
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("migraitions failed");

    handle_create_initial_admin(&config, &pool)
        .await
        .expect("Could not create admin user");
    start_server(pool).await
}

fn init_app(cfg: &mut web::ServiceConfig) {
    register_user_routes(cfg);
    register_project_routes(cfg);
}

async fn start_server(pool: Pool<Postgres>) -> std::io::Result<()> {
    HttpServer::new(move || {
        // TODO Configure this properly
        App::new()
            .configure(init_app)
            .wrap(Logger::new("%a %{User-Agent}i"))
            .app_data(web::Data::new(pool.clone()))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
