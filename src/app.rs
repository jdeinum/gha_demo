use crate::error::Result;
use crate::routes::health::health;
use crate::routes::latency::latency;
use crate::routes::v1::router::get_v1_router;
use crate::settings::Settings;
use anyhow::Context;
use axum::Router;
use axum::routing::get;
use rand::SeedableRng;
use rand::rngs::StdRng;
use sqlx::PgPool;
use tokio::net::TcpListener;
use tokio::signal::unix::{SignalKind, signal};
use tracing::info;

pub struct App {
    router: axum::Router,
    listener: TcpListener,
}

#[derive(Clone, Debug)]
pub struct AppState {
    pub db: PgPool,
    pub rng: StdRng,
}

impl App {
    pub async fn build(settings: Settings) -> Result<Self> {
        // app env changes what settings we're pulling
        let mode = std::env::var("APP_ENV").unwrap_or("local".to_string());
        info!("app mode: {mode}");

        // create the DB connection with pool settings
        let db = sqlx::pool::PoolOptions::new()
            .connect_with(settings.db.get_db_settings())
            .await
            .with_context(|| format!("connect to db with settings: {:?}", settings.db))?;

        // migrate the DB
        info!("migrating the db...");
        sqlx::migrate!("./migrations")
            .run(&db)
            .await
            .context("migrate db")?;

        // create the listener
        let listener = tokio::net::TcpListener::bind(settings.application.connection_string())
            .await
            .context("create tcp listener")?;

        // create rng
        let rng = StdRng::from_os_rng();

        // create our appstate
        let app_state = AppState {
            db: db.clone(),
            rng,
        };

        // create the router
        let router = Router::new()
            .route("/health", get(health))
            .route("/latency", get(latency))
            .nest("/v1", get_v1_router())
            .layer(tower_http::trace::TraceLayer::new_for_http())
            .with_state(app_state);

        Ok(Self { listener, router })
    }

    pub fn port(&self) -> Result<u16> {
        Ok(self.listener.local_addr().context("get local addr")?.port())
    }

    pub async fn run_until_stopped(self) -> Result<()> {
        info!("starting server on {:?}", self.listener.local_addr());
        axum::serve(self.listener, self.router)
            .with_graceful_shutdown(wait_for_term())
            .await
            .context("runing api")?;

        Ok(())
    }
}

async fn wait_for_term() {
    signal(SignalKind::terminate())
        .context("wait for SIGTERM")
        .unwrap()
        .recv()
        .await;
}
