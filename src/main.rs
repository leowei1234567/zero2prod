use secrecy::ExposeSecret;
use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::email_client::EmailClient;
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to get config");
    let sender = configuration
        .email_client
        .sender()
        .expect("Failed to get sender");

    let email_client = EmailClient::new(
        sender,
        configuration.email_client.base_url,
        configuration.email_client.authorization_token,
        configuration.email_client.timeout,
    );

    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );

    let connection_pool = PgPoolOptions::new()
        .connect_lazy(configuration.database.connection_string().expose_secret())
        .expect("Failed to connect to Postgres.");

    let listener = TcpListener::bind(address).expect("Failed to bind random port");

    run(listener, connection_pool, email_client)?.await
}
