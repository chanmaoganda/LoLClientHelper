mod league_app;
mod free_window;

use eframe::egui;
pub use league_app::LeagueApp;


fn main() -> Result<(), eframe::Error> {
    env_logger::init();
    free_window::hide_console_window();
    
    let main_win_opts = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([720., 460.]),
        persist_window: true,
        ..Default::default()
    };

    eframe::run_native(
        "Match History",
        main_win_opts,
        Box::new(move |cc| {
            Ok(Box::new(LeagueApp::new(cc)))
        }),
    )?;

    Ok(())
}