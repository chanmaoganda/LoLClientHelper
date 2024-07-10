mod league_app;
mod match_history_ui;

use std::sync::{Arc, RwLock};

use eframe::egui;
pub use league_app::LeagueApp;
use tokio::runtime::Runtime;

#[tokio::main]
async fn main() -> Result<(), eframe::Error> {
    let mut app = Arc::new(RwLock::new(LeagueApp::new()));
    let refresh_task = app.clone();
    let rt = Runtime::new().unwrap();
    let _ = rt.enter();

    std::thread::spawn(move || {
        rt.block_on(async {
            loop {
                refresh_task.write().unwrap().fetch_match_history().await.unwrap();
            }
        })
    });

    let main_win_opts = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([300., 480.]),
        persist_window: true,
        ..Default::default()
    };

    eframe::run_native(
        "Match History",
        main_win_opts,
        Box::new(move |_| {
            let inner_app = app.read().unwrap();
            Ok(Box::new())
        }),
    )?;

    Ok(())
}