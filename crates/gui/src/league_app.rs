use std::sync::Arc;

use eframe::egui::{self, mutex::RwLock, CentralPanel, Image, Vec2};
use league_model::GameHistoryQuery;

pub struct LeagueApp {
    pub lcu_client: Arc<RwLock<lcu_api::LcuClient>>,
    pub histories: Arc<RwLock<Vec<GameHistoryQuery>>>,
    pub filtered_histories: Arc<RwLock<Vec<GameHistoryQuery>>>,
    pub is_classic_mode: bool,
    pub max_game_count: u32,
    pub display_size: u32,
}

/// functional implement block
impl LeagueApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::customize_font(&cc.egui_ctx);
        Self {
            lcu_client: Arc::new(RwLock::new(lcu_api::LcuClient::new())),
            histories: Arc::new(RwLock::new(vec![])),
            filtered_histories: Arc::new(RwLock::new(vec![])),
            is_classic_mode: false,
            max_game_count: 20,
            display_size: 5,
        }
    }

    pub fn fetch_match_history(&mut self) -> anyhow::Result<bool> {
        log::debug!("Fetching match history");
        let rt = tokio::runtime::Runtime::new().unwrap();
        let histories_task = self.histories.clone();
        let filtered_histories_task = self.filtered_histories.clone();
        let lcu_task = self.lcu_client.clone();
        let game_count = self.max_game_count;
        rt.block_on(async move {
            let mut histories = histories_task.write();
            let mut filtered_history = filtered_histories_task.write();

            let expect_history = lcu_task.read().get_summoner_match_histories(game_count).await.unwrap();
            
            if expect_history.is_empty() {
                log::debug!("No match history found currently");
                return;
            }
            histories.clear();
            histories.extend(expect_history.clone());

            filtered_history.clear();
            filtered_history.extend(expect_history.clone());

            log::info!("History Updated!");
        });
        log::debug!("Match history size: {}", self.histories.read().len());
        Ok(self.histories.read().len() > 0)
    }

    fn filter_by_mode(&mut self) {
        let mut histories = self.filtered_histories.write();
        if !self.is_classic_mode {
            return;
        }
        log::debug!("filtering history by mode");
        histories.iter_mut().for_each(|query|{
            let games = &mut query.game_history.game_list;
            games.retain(|game| game.get_game_mode() == "CLASSIC");
            log::debug!("filtered game count: {}", games.len());
        });
    }
}

impl eframe::App for LeagueApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.set_font_style(ctx);
        egui_extras::install_image_loaders(ctx);
        CentralPanel::default().show(ctx, |ui| {
            self.render_header_options(ui);
            egui::ScrollArea::vertical().show(ui, |ui| {
                self.render_match_history(ui);
            });
        });

    }
}

/// public functions to render egui components
impl LeagueApp {
    pub fn render_header_options(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui.button("Refresh").highlight().clicked() {
                self.fetch_match_history().unwrap();

                self.filter_by_mode();
            }

            ui.checkbox(&mut self.is_classic_mode, "Classic Mode");
            ui.separator();

            ui.add(egui::Slider::new(&mut self.display_size, 1..=self.max_game_count).text("Display Count"));
        });
    }
    
    pub fn render_match_history(&self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if self.is_classic_mode {
                self.render_game_history(ui, &self.filtered_histories);
                return;
            }
            self.render_game_history(ui, &self.histories);
        });
        
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
    
    #[cfg(feature = "debug")]
    fn render_game(ui: &mut egui::Ui, game: &league_model::Game) {
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

    #[cfg(feature = "release")]
    fn render_game(ui: &mut egui::Ui, game: &league_model::Game) {
        let champion_icon_url = game.get_champion_icon_url();
        let (spell1_url, spell2_url) = game.get_summoner_spell_urls();
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.add(Image::new(champion_icon_url).fit_to_exact_size(Vec2::new(30., 30.)));
                ui.vertical(|ui| {
                    ui.add(Image::new(spell1_url).fit_to_exact_size(Vec2::new(15., 15.)));
                    ui.add(Image::new(spell2_url).fit_to_exact_size(Vec2::new(15., 15.)));
                });
            });
            ui.label(egui::RichText::new(game.get_date()));
            ui.label(egui::RichText::new(game.get_kda_result()).color(egui::Color32::DARK_BLUE));
            if game.get_win_status() {
                ui.label(egui::RichText::new("Win").color(egui::Color32::GREEN));
            } else {
                ui.label(egui::RichText::new("Lose").color(egui::Color32::RED));
            }
        });
    }

    fn render_game_history(&self, ui: &mut egui::Ui, histories: &Arc<RwLock<Vec<GameHistoryQuery>>>) {
        ui.horizontal(|ui| {
            static PADDING: f32 = 4.;
            for (slot, history) in histories.read().iter().enumerate() {
                log::debug!("rendering player {}", slot + 1);
    
                ui.vertical(|ui| {
                    let games = &history.game_history.game_list;
                    ui.add_space(PADDING);
                    let header = egui::RichText::new(format!("player {}: ", slot + 1))
                        .font(egui::FontId::new(14.0, egui::FontFamily::Proportional));
                    ui.label(header);
                    ui.add_space(PADDING);
        
                    for (index, game) in games.iter().enumerate() {
                        if index == self.display_size as usize {
                            break;
                        }
                        Self::render_game(ui, game);
                    }
                });
                ui.separator();
            }
        });
    }
}
