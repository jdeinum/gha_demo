use anyhow::Context;
use anyhow::Result;
use criterion::{Criterion, criterion_group, criterion_main};
use gha_demo::{
    App,
    settings::{DbSettings, get_settings},
};
use secrecy::SecretString;
use sqlx::Executor;
use sqlx::{Connection, PgConnection, PgPool};
use std::hint::black_box;
use uuid::Uuid;

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
    pub api_client: reqwest::Client,
}

impl TestApp {
    pub fn new(address: String, db_pool: PgPool, api_client: reqwest::Client) -> Self {
        Self {
            address,
            db_pool,
            api_client,
        }
    }
}

pub async fn spawn_app() -> Result<TestApp> {
    // Randomise configuration to ensure test isolation
    let configuration = {
        let mut c = get_settings().context("read settings for test")?;
        // Use a different database for each test case
        c.db.database = Uuid::new_v4().to_string();
        // Use a random OS port
        c.application.port = 0;

        c
    };

    // configure our DB
    configure_db(&configuration.db)
        .await
        .context("configure db")?;

    // Launch the application as a background task
    let application = App::build(configuration.clone())
        .await
        .context("build app in test")?;

    // get the address and port we should be connecting to
    let port = application
        .port()
        .context("get application port for test")?;
    let address = format!("http://localhost:{}", port);

    // spawn our app as a background task
    let _ = tokio::spawn(application.run_until_stopped());

    // create our request client
    let api_client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .context("build http client")?;

    // create the connection to our database
    let db_pool = PgPool::connect_with(configuration.db.get_db_settings())
        .await
        .context("connect to db")?;

    let test_app = TestApp {
        db_pool,
        address,
        api_client,
    };

    Ok(test_app)
}

async fn configure_db(settings: &DbSettings) -> Result<()> {
    // Create database
    let maintenance_settings = DbSettings {
        database: "postgres".to_string(),
        username: "postgres".to_string(),
        password: SecretString::new("password".to_string().into()),
        ..settings.clone()
    };

    let mut connection = PgConnection::connect_with(&maintenance_settings.get_db_settings())
        .await
        .with_context(|| {
            format!("connect to postgres db to create db with settings {maintenance_settings:?}",)
        })?;

    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, settings.database).as_str())
        .await
        .context("create test db")?;

    // Migrate database
    let connection_pool = PgPool::connect_with(settings.get_db_settings())
        .await
        .context("migrate test db")?;

    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");
    Ok(())
}

fn criterion_benchmark(c: &mut Criterion) {
    // send requests to the latency endpoint

    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
