use sqlx::PgPool;
use std::net::TcpListener;

use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;
use zero2prod::telemetry::init_tracing;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    init_tracing();

    let configuration = get_configuration().expect("Failed to read configuration.");
    let pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to PG.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener, pool)?.await
}