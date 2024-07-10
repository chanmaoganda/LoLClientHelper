use std::sync::Arc;

use eframe::egui::{self, mutex::RwLock, CentralPanel, Image, Label, ScrollArea, Vec2};
use league_model::GameHistoryQuery;

pub struct LeagueApp {
    pub lcu_client: Arc<RwLock<lcu_api::LcuClient>>,
    pub histories: Arc<RwLock<Vec<GameHistoryQuery>>>,
}

impl LeagueApp {
    pub fn new() -> Self {
        Self {
            lcu_client: Arc::new(RwLock::new(lcu_api::LcuClient::new())),
            histories: Arc::new(RwLock::new(vec![])),
        }
    }

    pub fn fetch_match_history(&mut self) -> anyhow::Result<bool> {
        log::info!("Fetching match history");
        let rt = tokio::runtime::Runtime::new().unwrap();
        let histories_task = self.histories.clone();
        let lcu_task = self.lcu_client.clone();
        rt.block_on(async move {
            let mut histories = histories_task.write();
            histories.clear();
            let expect_history = lcu_task.read().get_summoner_match_histories().await.unwrap();
            histories.extend(expect_history);
        });
        log::info!("Done fetching match history");
        log::info!("Match history size: {}", self.histories.read().len());
        Ok(self.histories.read().len() > 0)
    }


}

impl eframe::App for LeagueApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui_extras::install_image_loaders(ctx);
        CentralPanel::default().show(ctx, |ui| {
            self.render_refresh_button(ui);
            ScrollArea::vertical().show(ui, |ui| {
                self.render_match_history(ui);
            });
        });

    }
}

impl LeagueApp {
    pub fn render_refresh_button(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui.button("Refresh Match History").highlight().clicked() {
                self.fetch_match_history().unwrap();
            }
            ui.separator();
        });
    }
    
    pub fn render_match_history(&self, ui: &mut egui::Ui) {
        static PADDING: f32 = 4.;
        for (slot, history) in self.histories.read().iter().enumerate() {
            log::debug!("rendering player {}", slot + 1);
            let games = &history.game_history.game_list;
            ui.add_space(PADDING);
            ui.add(Label::new(format!("player {} game history: ", slot + 1)));
            ui.add_space(PADDING);
            for (index, game) in games.iter().enumerate() {
                if index == 5 {
                    break;
                }
                let champion_icon_url = game.get_champion_icon_url();
                ui.horizontal(|ui| {
                    ui.add(Image::new(champion_icon_url).fit_to_exact_size(Vec2::new(30., 30.)));
                    ui.add_space(10.);
                    ui.vertical(|ui| {
                        ui.add(Label::new(game.get_game_info()));
                        ui.add(Label::new(game.get_player_info()));
                        ui.add(Label::new(game.get_kda_result()));
                    });
                });
                ui.add_space(3.);
            }
            ui.separator();
        }
    }

}
