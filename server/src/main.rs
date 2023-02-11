use std::sync::Arc;
use std::sync::Mutex;

use log::info;
use tokio::sync::{broadcast, watch};

use graphql::graphql_api;
use ps_move::effects::LedEffectDetails;

use crate::logger::setup_logger;
use crate::ps_move::controller::PsMoveController;
use crate::ps_move::models::ButtonState;
use crate::tasks::models::*;

mod graphql;
mod logger;
mod ps_move;
mod spawn_tasks;
mod tasks;

#[tokio::main]
async fn main() {
    setup_logger();

    let (effect_tx, effect_rx) = broadcast::channel(32);
    let (ctrl_tx, ctrl_rx) = watch::channel(ControllerChange::from_button(
        &Button::Move,
        &ButtonState::Down,
    ));
    let controllers = Arc::new(Mutex::new(Vec::<PsMoveController>::new()));

    let mut shutdown_command = spawn_tasks::run_move(effect_rx, ctrl_tx, &controllers).await;
    graphql_api::start(Arc::new(effect_tx), Mutex::new(ctrl_rx), controllers).await;

    info!("Shutting down...");
    shutdown_command.shutdown().await
}
