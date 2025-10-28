// some important fixes here
// and again
// @deinum - testing release-plz version bump
pub(crate) mod app;
pub(crate) mod error;
pub(crate) mod routes;
pub(crate) mod run;
pub(crate) mod telemetry;

// main entrypoint to lib
pub use run::run;

// tests need access to these
pub mod settings;
pub mod types;
pub use app::App;
