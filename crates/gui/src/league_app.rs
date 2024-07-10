use std::sync::Arc;

use eframe::egui::{self, mutex::RwLock, CentralPanel, Label, Sense, Separator};
use league_model::GameHistoryQuery;

pub struct LeagueApp {
    lcu_client: lcu_api::LcuClient,
    histories: Arc<RwLock<Vec<GameHistoryQuery>>>,
}

impl LeagueApp {
    pub fn new() -> Self {
        Self {
            lcu_client: lcu_api::LcuClient::new(),
            histories: Arc::new(RwLock::new(vec![])),
        }
    }

    pub async fn fetch_match_history(&mut self) -> anyhow::Result<bool> {
        let histories = self.lcu_client.get_summoner_match_histories().await?;
        self.histories = Arc::new(RwLock::new(histories));
        Ok(self.histories.read().len() > 0)
    }
}

impl eframe::App for LeagueApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            render_match_history(ui, &self.histories.read());
        });
    }
}

pub fn render_match_history(ui: &mut egui::Ui, histories: &Vec<league_model::GameHistoryQuery>) {
    static PADDING: f32 = 4.;
    for history in histories {
        ui.add_space(PADDING);
        let label = Label::new("Match History").sense(Sense::click());
        ui.add(label);
        let user = Label::new(format!("Summoner {}", history.account_id));
        ui.add(user);
        ui.add(Separator::default());
    }
}