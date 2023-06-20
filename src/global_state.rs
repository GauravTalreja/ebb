use perseus::{prelude::*, state::GlobalStateCreator};
use serde::{Deserialize, Serialize};

pub fn get_global_state_creator() -> GlobalStateCreator {
    GlobalStateCreator::new().build_state_fn(get_build_state)
}

#[derive(Serialize, Deserialize, ReactiveState)]
#[rx(alias = "AppStateRx")]
pub struct AppState {
    pub theme: String,
}

#[engine_only_fn]
async fn get_build_state() -> AppState {
    AppState {
        theme: "dark".to_string(),
    }
}
