use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;

use tokio::sync::watch::Sender;
use tokio::time;

use crate::ControllerChange;
use crate::ps_move::controller::PsMoveController;
use crate::ps_move::models::ButtonState;
use crate::spawn_tasks::ShutdownSignal;

const INTERVAL_DURATION: Duration = Duration::from_millis(10);

pub async fn run(
    controllers: Arc<Mutex<Vec<PsMoveController>>>,
    ctrl_tx: Sender<ControllerChange>,
    mut shutdown_signal: ShutdownSignal,
) {
    let mut interval = time::interval(INTERVAL_DURATION);

    while !shutdown_signal.check_is_shutting_down() {
        interval.tick().await;

        let mut controllers = controllers.lock().unwrap();
        let mut failed_addresses = Vec::<String>::new();

        controllers
            .iter_mut()
            .for_each(|controller| match controller.update() {
                Ok(_) => {
                    controller.button_state.iter().for_each(|btn| match btn.1 {
                        ButtonState::Pressed | ButtonState::Released => {
                            tracing::info!(
                                "Controller {} button {} changed to {}",
                                controller.bt_address,
                                btn.0,
                                btn.1
                            );

                            ctrl_tx
                                .send(ControllerChange::from_button(btn.0, btn.1))
                                .unwrap();
                        }
                        _ => {}
                    });
                }
                Err(_) => {
                    let bt_address = &controller.bt_address;

                    tracing::info!(
                        "Controller disconnected during update. ('{}' by {})",
                        *bt_address,
                        controller.connection_type
                    );

                    failed_addresses.push(bt_address.clone());
                }
            });

        controllers.retain(|c| !failed_addresses.contains(&c.bt_address));
    }
}
