use juniper::FieldResult;

use crate::graphql::schema::Context;
use crate::graphql::schema_response::{Controller, HealthStatus};

mod api {
    pub use crate::ps_move::effects::*;
}

mod graphql {
    pub use crate::graphql::schema_response::*;
}


pub struct QueryRoot;

#[juniper::graphql_object(Context = Context)]
impl QueryRoot {
    #[graphql(description = "Check the service health")]
    fn health(_ctx: &Context) -> FieldResult<HealthStatus> {
        Ok(HealthStatus::Ok)
    }

    #[graphql(description = "Lists all connected controllers")]
    async fn controllers(_ctx: &Context) -> FieldResult<Vec<Controller>> {
        let controllers = _ctx.controllers.lock().unwrap();

        Ok(controllers.iter().map(|ctl| {
            Controller {
                address: ctl.bt_address.clone(),
                battery_level: ctl.battery,
                connection_type: ctl.connection_type,
                current_led_effect: match ctl.led_effect.details {
                    api::LedEffectDetails::Off => { graphql::LedEffectType::Off }
                    api::LedEffectDetails::Static { .. } => { graphql::LedEffectType::Static }
                    api::LedEffectDetails::Breathing { .. } => { graphql::LedEffectType::Breathing }
                    api::LedEffectDetails::Rainbow { .. } => { graphql::LedEffectType::Rainbow }
                    api::LedEffectDetails::Blink { .. } => { graphql::LedEffectType::Blink }
                    api::LedEffectDetails::Shift { .. } => { graphql::LedEffectType::Shift }
                },
                current_rumble_effect: match ctl.rumble_effect.details {
                    api::RumbleEffectDetails::Off => { graphql::RumbleEffectType::Off }
                    api::RumbleEffectDetails::Static { .. } => { graphql::RumbleEffectType::Static }
                    api::RumbleEffectDetails::Breathing { .. } => { graphql::RumbleEffectType::Breathing }
                    api::RumbleEffectDetails::Blink { .. } => { graphql::RumbleEffectType::Blink }
                },
            }
        })
            .collect())
    }
}
