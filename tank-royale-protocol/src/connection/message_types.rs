use serde::{Deserialize, Serialize};
use crate::connection::enums::BotEvent;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BotAddress {
    pub host: String,
    pub port: u16,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BotDeathEvent {
    pub turn_number: i32,
    pub victim_id: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BotHandshake {
    pub authors: Vec<String>,
    pub country_codes: Option<Vec<String>>,
    pub description: String,
    pub game_types: Vec<String>,
    pub homepage: String,
    pub initial_position: Option<InitialPosition>,
    pub is_droid: bool,
    pub name: String,
    pub platform: String,
    pub programming_lang: String,
    pub secret: Option<String>,
    pub session_id: String,
    pub team_id: Option<i32>,
    pub team_name: Option<String>,
    pub team_version: Option<String>,
    pub version: String,
}

impl BotHandshake {
    pub fn from_bot_info(session_id: &String, bot_info: crate::bot::BotInfo, team_info: Option<crate::bot::TeamInfo>, is_droid: bool, server_secret: String) -> Self {
        let (team_id, team_name, team_version) = if let Some(team_info) = team_info {
            (Some(team_info.id), Some(team_info.name), Some(team_info.version))
        } else { (None, None, None) };
        Self {
            authors: bot_info.authors,
            country_codes: bot_info.country_codes,
            description: bot_info.description,
            game_types: bot_info.game_types,
            homepage: bot_info.homepage,
            initial_position: bot_info.initial_position,
            is_droid,
            name: bot_info.name,
            platform: bot_info.platform,
            programming_lang: bot_info.programming_lang,
            secret,
            session_id: (*session_id.clone()).parse().unwrap(),
            team_id,
            team_name,
            team_version,
            version: bot_info.version,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BotHitBotEvent {
    pub turn_number: i32,
    pub bot_id: i32,
    pub victim_id: i32,
    pub energy: f32,
    pub rammed: bool,
    pub x: f32,
    pub y: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BotHitWallEvent {
    pub turn_number: i32,
    pub victim_id: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BotHandshakeInfo {
    pub host: String,
    pub port: u16,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BotIntent {
    pub adjust_gun_for_body_turn: Option<bool>,
    pub adjust_radar_for_body_turn: Option<bool>,
    pub adjust_radar_for_gun_turn: Option<bool>,
    pub body_color: Option<Color>,
    pub bullet_color: Option<Color>,
    pub fire_assist: Option<bool>,
    pub firepower: Option<f32>,
    pub gun_color: Option<Color>,
    pub gun_turn_rate: Option<f32>,
    pub radar_color: Option<Color>,
    pub radar_turn_rate: Option<f32>,
    pub rescan: Option<bool>,
    pub scan_color: Option<Color>,
    pub stderr: Option<String>,
    pub stdout: Option<String>,
    pub target_speed: Option<f32>,
    pub team_messages: Option<Vec<TeamMessage>>,
    pub tracks_color: Option<Color>,
    pub turn_rate: Option<f32>,
    pub turret_color: Option<Color>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BotListUpdate {
    pub bots: Vec<BotHandshakeInfo>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BotStateWithId {
    pub id: i32,
    pub session_id: String,
    pub stderr: Option<String>,
    pub stdout: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BotState {
    pub body_color: Option<Color>,
    pub bullet_color: Option<Color>,
    pub direction: f32,
    pub energy: f32,
    pub gun_color: Option<Color>,
    pub gun_direction: f32,
    pub gun_heat: f32,
    pub gun_turn_rate: f32,
    pub is_droid: bool,
    pub radar_color: Option<Color>,
    pub radar_direction: f32,
    pub radar_sweep: f32,
    pub radar_turn_rate: f32,
    pub scan_color: Option<Color>,
    pub speed: f32,
    pub tracks_color: Option<Color>,
    pub turn_rate: f32,
    pub turret_color: Option<Color>,
    pub x: f32,
    pub y: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BulletFiredEvent {
    pub turn_number: i32,
    pub bullet: BulletState,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BulletHitBotEvent {
    pub turn_number: i32,
    pub bullet: BulletState,
    pub damage: f32,
    pub energy: f32,
    pub victim_id: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BulletHitBulletEvent {
    pub turn_number: i32,
    pub bullet: BulletState,
    pub hit_bullet: BulletState,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BulletHitWallEvent {
    pub turn_number: i32,
    pub bullet: BulletState,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BulletState {
    pub bullet_id: i32,
    pub color: Color,
    pub direction: f32,
    pub owner_id: i32,
    pub power: f32,
    pub x: f32,
    pub y: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChangeTps {
    pub tps: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Color(String);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ControllerHandshake {
    pub author: String,
    pub name: String,
    pub secret: Option<String>,
    pub session_id: String,
    pub version: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameAbortedEvent {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameEndedEventForBot {
    pub number_of_rounds: i32,
    pub results: ResultsForBot,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameEndedEventForObserver {
    pub number_of_rounds: i32,
    pub results: Vec<ResultsForObserver>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameSetup {
    pub arena_height: i32,
    pub arena_width: i32,
    pub default_turns_per_second: i32,
    pub game_type: String,
    pub gun_cooling_rate: f32,
    pub is_arena_height_locked: bool,
    pub is_arena_width_locked: bool,
    pub is_gun_cooling_rate_locked: bool,
    pub is_max_inactivity_turns_locked: bool,
    pub is_max_number_of_participants_locked: bool,
    pub is_min_number_of_participants_locked: bool,
    pub is_number_of_rounds_locked: bool,
    pub is_ready_timeout_locked: bool,
    pub is_turn_timeout_locked: bool,
    pub max_inactivity_turns: i32,
    pub max_number_of_participants: Option<i32>,
    pub min_number_of_participants: i32,
    pub number_of_rounds: i32,
    pub ready_timeout: i32,
    pub turn_timeout: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameStartedEventForBot {
    pub game_setup: GameSetup,
    pub my_id: i32,
    pub start_direction: f32,
    pub start_x: f32,
    pub start_y: f32,
    pub teammate_ids: Vec<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameStartedEventForObserver {
    pub game_setup: GameSetup,
    pub participants: Vec<Participant>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HitByBulletEvent {
    pub turn_number: i32,
    pub bullet: BulletState,
    pub damage: f32,
    pub energy: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InitialPosition {
    pub direction: Option<f32>,
    pub x: Option<f32>,
    pub y: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ObserverHandshake {
    pub author: String,
    pub name: String,
    pub secret: Option<String>,
    pub session_id: String,
    pub version: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Participant {
    pub authors: Vec<String>,
    pub country_codes: Vec<String>,
    pub description: String,
    pub game_types: Vec<String>,
    pub homepage: Option<String>,
    pub id: i32,
    pub initial_position: Option<InitialPosition>,
    pub is_droid: bool,
    pub name: String,
    pub platform: String,
    pub programming_lang: String,
    pub session_id: String,
    pub team_id: Option<i32>,
    pub team_name: Option<String>,
    pub team_version: Option<String>,
    pub version: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResultsForBot {
    pub bullet_damage: i32,
    pub bullet_kill_bonus: i32,
    pub first_places: i32,
    pub last_survivor_bonus: i32,
    pub ram_damage: i32,
    pub ram_kill_bonus: i32,
    pub rank: i32,
    pub second_places: i32,
    pub survival: i32,
    pub third_places: i32,
    pub total_score: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResultsForObserver {
    pub id: i32,
    pub name: String,
    pub version: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RoundEndedEventForBot {
    pub results: ResultsForBot,
    pub round_number: i32,
    pub turn_number: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RoundEndedEventForObserver {
    pub results: Vec<ResultsForObserver>,
    pub round_number: i32,
    pub turn_number: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RoundStartedEvent {
    pub round_number: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ScannedBotEvent {
    pub turn_number: i32,
    pub direction: f32,
    pub energy: f32,
    pub scanned_bot_id: i32,
    pub scanned_by_bot_id: i32,
    pub speed: f32,
    pub x: f32,
    pub y: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServerHandshake {
    pub game_setup: Option<GameSetup>,
    pub game_types: Vec<String>,
    pub name: String,
    pub session_id: String,
    pub variant: String,
    pub version: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SkippedTurnEvent {
    pub turn_number: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StartGame {
    pub bot_addresses: Vec<BotAddress>,
    pub game_setup: GameSetup,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TeamMessageEvent {
    pub turn_number: i32,
    pub message: String,
    pub message_type: String,
    pub sender_id: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TeamMessage {
    pub message: String,
    pub message_type: String,
    pub receiver_id: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TickEventForBot {
    pub turn_number: i32,
    pub bot_state: BotState,
    pub bullet_states: Vec<BulletState>,
    pub enemy_count: i32,
    pub events: Vec<BotEvent>,
    pub round_number: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TickEventForObserver {
    pub turn_number: i32,
    pub bot_states: Vec<BotStateWithId>,
    pub bullet_states: Vec<BulletState>,
    pub events: Vec<BotEvent>,
    pub round_number: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TpsChangedEvent {
    pub tps: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WonRoundEvent {
    pub turn_number: i32,
}