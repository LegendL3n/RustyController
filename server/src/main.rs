use std::{io::Error, sync::Arc};

use log::info;
use tokio::signal;
use tokio::sync::watch;

use graphql::graphql_api;
use ps_move_api::LedEffect;

mod graphql;
mod move_task;
mod ps_move_api;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let (tx, rx) = watch::channel(LedEffect::Off);

    let task = tokio::spawn(move_task::run_move(rx));
    graphql_api::start(Arc::new(tx)).await;

    info!("Shutting down...");
}
