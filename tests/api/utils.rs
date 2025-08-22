use anyhow::Context;
use anyhow::Result;
use gha_demo::App;
use gha_demo::settings::{DbSettings, get_settings};
use gha_demo::types::v1::types::Cat;
use gha_demo::types::v1::types::EyeColor;
use secrecy::SecretString;
use sqlx::Connection;
use sqlx::Executor;
use sqlx::PgConnection;
use sqlx::PgPool;
use std::sync::LazyLock;
use uuid::Uuid;

static TRACING: LazyLock<()> = LazyLock::new(|| {
    if std::env::var("TESTING_LOG").is_ok() {
        tracing_subscriber::fmt::init();
    }
});

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
    pub api_client: reqwest::Client,
}

pub async fn spawn_app() -> Result<TestApp> {
    // initialize tracing for our tests
    LazyLock::force(&TRACING);

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

pub async fn create_two_cats(pool: &sqlx::PgPool) -> Result<[Cat; 2]> {
    // Example data
    let cat1 = Cat {
        name: "Whiskers".to_string(),
        cool_cat_club_id: Uuid::new_v4(),
        age: 2,
        eye_color: EyeColor::Blue,
    };

    let cat2 = Cat {
        name: "Mittens".to_string(),
        cool_cat_club_id: Uuid::new_v4(),
        age: 4,
        eye_color: EyeColor::Brown,
    };

    // Create both cats
    let _ = cat1.write_to_db(pool).await?;

    let _ = cat2.write_to_db(pool).await?;

    Ok([cat1, cat2])
}
