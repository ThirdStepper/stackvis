use std::sync::{Arc, Mutex};

mod engine;
mod sorting_algorithms;
mod stats;
mod ui;

use crate::engine::EngineSharedState;
use crate::ui::ui::SortVisApp;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions::default();

    eframe::run_native(
        "VisSort",
        native_options,
        Box::new(|creation_context| {
            let shared_state = Arc::new(Mutex::new(EngineSharedState::default()));
            let app = SortVisApp::new(creation_context, shared_state);
            Ok(Box::new(app))
        }),
    )
}

