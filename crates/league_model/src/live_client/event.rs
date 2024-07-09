use serde::Deserialize;

use super::Drake;

use super::player::Team;


#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Events {
    #[serde(rename = "Events")]
    pub events: Vec<Event>,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "EventName")]
pub enum Event {
    GameStart(GameStart),
    GameEnd(GameEnd),
    MinionsSpawning(MinionsSpawning),
    FirstBrick(FirstBrick),
    FirstBlood(FirstBlood),
    TurretKilled(TurretKilled),
    InhibKilled(InhibKilled),
    InhibRespawningSoon(InhibRespawningSoon),
    InhibRespawned(InhibRespawned),
    DragonKill(DragonKill),
    HeraldKill(HeraldKill),
    BaronKill(BaronKill),
    ChampionKill(ChampionKill),
    Multikill(Multikill),
    Ace(Ace),
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct GameStart {
    #[serde(rename = "EventID")]
    pub event_id: usize,
    #[serde(rename = "EventTime")]
    pub event_time: f64,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct GameEnd {
    #[serde(rename = "EventID")]
    pub event_id: usize,
    #[serde(rename = "EventTime")]
    pub event_time: f64,
    #[serde(rename = "Result")]
    pub result: GameResult,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub enum GameResult {
    Win,
    Loss,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct MinionsSpawning {
    #[serde(rename = "EventID")]
    pub event_id: usize,
    #[serde(rename = "EventTime")]
    pub event_time: f64,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct TurretKilled {
    #[serde(rename = "EventID")]
    pub event_id: usize,
    #[serde(rename = "EventTime")]
    pub event_time: f64,
    #[serde(rename = "KillerName")]
    pub killer_name: String,
    #[serde(rename = "TurretKilled")]
    pub turret_killed: String,
    #[serde(rename = "Assisters")]
    pub assisters: Vec<String>,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct InhibKilled {
    #[serde(rename = "EventID")]
    pub event_id: usize,
    #[serde(rename = "EventTime")]
    pub event_time: f64,
    #[serde(rename = "KillerName")]
    pub killer_name: String,
    #[serde(rename = "InhibKilled")]
    pub inhib_killed: String,
    #[serde(rename = "Assisters")]
    pub assisters: Vec<String>,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct InhibRespawningSoon {
    #[serde(rename = "EventID")]
    pub event_id: usize,
    #[serde(rename = "EventTime")]
    pub event_time: f64,
    #[serde(rename = "InhibRespawningSoon")]
    pub inhib_respawning_soon: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct InhibRespawned {
    #[serde(rename = "EventID")]
    pub event_id: usize,
    #[serde(rename = "EventTime")]
    pub event_time: f64,
    #[serde(rename = "InhibRespawned")]
    pub inhib_respawned: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct DragonKill {
    #[serde(rename = "EventID")]
    pub event_id: usize,
    #[serde(rename = "EventTime")]
    pub event_time: f64,
    #[serde(rename = "KillerName")]
    pub killer_name: String,
    #[serde(rename = "Assisters")]
    pub assisters: Vec<String>,
    #[serde(rename = "DragonType")]
    pub dragon_type: Drake,
    #[serde(rename = "Stolen")]
    pub stolen: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct FirstBrick {
    #[serde(rename = "EventID")]
    pub event_id: usize,
    #[serde(rename = "EventTime")]
    pub event_time: f64,
    #[serde(rename = "KillerName")]
    pub killer_name: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct FirstBlood {
    #[serde(rename = "EventID")]
    pub event_id: usize,
    #[serde(rename = "EventTime")]
    pub event_time: f64,
    #[serde(rename = "Recipient")]
    pub recipient: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct HeraldKill {
    #[serde(rename = "EventID")]
    pub event_id: usize,
    #[serde(rename = "EventTime")]
    pub event_time: f64,
    #[serde(rename = "KillerName")]
    pub killer_name: String,
    #[serde(rename = "Assisters")]
    pub assisters: Vec<String>,
    #[serde(rename = "Stolen")]
    pub stolen: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Ace {
    #[serde(rename = "EventID")]
    pub event_id: usize,
    #[serde(rename = "EventTime")]
    pub event_time: f64,
    #[serde(rename = "Acer")]
    pub acer: String,
    #[serde(rename = "AcingTeam")]
    pub acing_team: Team,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Multikill {
    #[serde(rename = "EventID")]
    pub event_id: usize,
    #[serde(rename = "EventTime")]
    pub event_time: f64,
    #[serde(rename = "KillerName")]
    pub killer_name: String,
    #[serde(rename = "KillStreak")]
    pub kill_streak: u8,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct ChampionKill {
    #[serde(rename = "EventID")]
    pub event_id: usize,
    #[serde(rename = "EventTime")]
    pub event_time: f64,
    #[serde(rename = "KillerName")]
    pub killer_name: String,
    #[serde(rename = "VictimName")]
    pub victim_name: String,
    #[serde(rename = "Assisters")]
    pub assisters: Vec<String>,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct BaronKill {
    #[serde(rename = "EventID")]
    pub event_id: usize,
    #[serde(rename = "EventTime")]
    pub event_time: f64,
    #[serde(rename = "KillerName")]
    pub killer_name: String,
    #[serde(rename = "Assisters")]
    pub assisters: Vec<String>,
    #[serde(rename = "Stolen")]
    pub stolen: String,
}