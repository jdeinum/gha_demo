use crate::{
    app::App,
    settings::get_settings,
    telemetry::{get_subscriber, init_subscriber},
};
use tracing::error;

pub async fn run() {
    // get the settings
    let settings = match get_settings() {
        Ok(s) => s,
        Err(e) => {
            println!("Error getting settings: {e:?}");
            return;
        }
    };

    // initialize tracing
    let subscriber = match get_subscriber() {
        Ok(s) => s,
        Err(e) => {
            println!("Error getting subscriber: {e:?}");
            return;
        }
    };

    if let Err(e) = init_subscriber(subscriber) {
        println!("Error initializing subscriber {e:?}");
        return;
    }

    // build app
    let app = match App::build(settings).await {
        Ok(a) => a,
        Err(e) => {
            error!("Error building app: {e:?}");
            return;
        }
    };

    // run app
    match app.run_until_stopped().await {
        Ok(_) => unreachable!(),
        Err(e) => {
            error!("Error running app: {e:?}");
        }
    }
}
