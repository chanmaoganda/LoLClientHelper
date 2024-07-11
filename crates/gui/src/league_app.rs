use std::sync::Arc;

use eframe::egui::{self, mutex::RwLock, CentralPanel, Image, Label, ScrollArea, Vec2};
use league_model::GameHistoryQuery;

pub struct LeagueApp {
    pub lcu_client: Arc<RwLock<lcu_api::LcuClient>>,
    pub histories: Arc<RwLock<Vec<GameHistoryQuery>>>,
}

/// functional implement block
impl LeagueApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::customize_font(&cc.egui_ctx);
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
            let expect_history = lcu_task.read().get_summoner_match_histories().await.unwrap();
            
            if expect_history.is_empty() {
                log::debug!("No match history found currently");
                return;
            }
            histories.clear();
            histories.extend(expect_history);
        });
        log::info!("Done fetching match history");
        log::info!("Match history size: {}", self.histories.read().len());
        Ok(self.histories.read().len() > 0)
    }


}

impl eframe::App for LeagueApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.set_font_style(ctx);

        egui_extras::install_image_loaders(ctx);
        CentralPanel::default().show(ctx, |ui| {
            self.render_refresh_button(ui);
            ScrollArea::vertical().show(ui, |ui| {
                self.render_match_history(ui);
            });
        });

    }
}

/// public functions to render egui components
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
            let header = egui::RichText::new(format!("player {} game history: ", slot + 1))
                .font(egui::FontId::new(14.0, egui::FontFamily::Proportional));
            ui.label(header);
            ui.add_space(PADDING);

            for (index, game) in games.iter().enumerate() {
                if index == 5 {
                    break;
                }
                self.render_game(ui, game);
            }
            ui.separator();
        }
    }

}

/// private functions to render egui components
impl LeagueApp {
    fn set_font_style(&self, ctx: &egui::Context) {
        let mut style = (*ctx.style()).clone();
        style.text_styles = [
            (egui::TextStyle::Heading, egui::FontId::new(12.0, egui::FontFamily::Proportional)),
            (egui::TextStyle::Body, egui::FontId::new(11.0, egui::FontFamily::Proportional)),
            (egui::TextStyle::Monospace, egui::FontId::new(8.0, egui::FontFamily::Proportional)),
            (egui::TextStyle::Button, egui::FontId::new(14.0, egui::FontFamily::Proportional)),
            (egui::TextStyle::Small, egui::FontId::new(10.0, egui::FontFamily::Proportional)),
        ]
        .into();
        ctx.set_style(style);
    }

    fn customize_font(ctx: &egui::Context) {
        let mut fonts = egui::FontDefinitions::default();
        fonts.font_data.insert("consola".to_owned(),
            egui::FontData::from_static(include_bytes!("../../../assets/consola_mono/ConsolaMono-Book.ttf")));
        fonts.families.entry(
            egui::FontFamily::Proportional).or_default()
            .insert(0, "consola".to_owned());
        fonts.families.entry(
            egui::FontFamily::Monospace).or_default()
            .insert(0, "consola".to_owned());
        ctx.set_fonts(fonts);
    }

    fn render_game(&self, ui: &mut egui::Ui, game: &league_model::Game) {
        let champion_icon_url = game.get_champion_icon_url();
        let (spell1_url, spell2_url) = game.get_summoner_spell_urls();
        ui.horizontal(|ui| {
            
            ui.vertical(|ui| {
                ui.add(Image::new(champion_icon_url).fit_to_exact_size(Vec2::new(30., 30.)));
                ui.horizontal(|ui| {
                    ui.add(Image::new(spell1_url).fit_to_exact_size(Vec2::new(15., 15.)));
                    ui.add(Image::new(spell2_url).fit_to_exact_size(Vec2::new(15., 15.)));
                });
            });

            ui.add_space(10.);
            ui.vertical(|ui| {
                ui.add(Label::new(game.get_game_info()));
                ui.add(Label::new(game.get_name_title()));
                ui.add(Label::new(game.get_kda_result()));
            });
        });
        ui.add_space(3.);
    }
}
