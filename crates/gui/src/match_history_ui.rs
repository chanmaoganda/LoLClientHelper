use eframe::egui;
pub use super::LeagueApp;


pub async fn match_history_ui() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = LeagueApp::new();

    app.fetch_match_history().await.unwrap();

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
            Ok(Box::new(app))
        }),
    )?;

    Ok(())
}

pub async fn run_match_history_ui() {
    
}