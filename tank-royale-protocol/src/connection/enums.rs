use super::message_types::*;
use serde::{self, Serialize, Deserialize};

//pub enum MessageType {
//    BotHandshake(BotHandshake),
//    BotIntent(BotIntent),
//    BotListUpdate(BotListUpdate),
//    BotReady,
//    ChangeTps(ChangeTps),
//    ControllerHandshake(ControllerHandshake),
//    Event(Event),
//    GameAbortedEvent(GameAbortedEvent),
//    GameEndedEventForBot(GameEndedEventForBot),
//    GameEndedEventForObserver(GameEndedEventForObserver),
//    GamePausedEventForObserver,
//    GameResumedEventForObserver,
//    GameStartedEventForBot(GameStartedEventForBot),
//    GameStartedEventForObserver(GameStartedEventForObserver),
//    NextTurn,
//    ObserverHandshake(ObserverHandshake),
//    PauseGame,
//    ResumeGame,
//    RoundEndedEventForBot(RoundEndedEventForBot),
//    RoundEndedEventForObserver(RoundEndedEventForObserver),
//    RoundStartedEvent(RoundStartedEvent),
//    ServerHandshake(ServerHandshake),
//    StartGame(StartGame),
//    StopGame,
//    TpsChangedEvent(TpsChangedEvent),
//}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum MessageType {
    BotHandshake(BotHandshake),
    ControllerHandshake(ControllerHandshake),
    ObserverHandshake(ObserverHandshake),
    ServerHandshake(ServerHandshake),

    BotReady,
    BotIntent(BotIntent),
    BotInfo(BotHandshakeInfo),
    BotListUpdate(BotListUpdate),

    GameStartedEventForBot(GameStartedEventForBot),
    GameStartedEventForObserver(GameStartedEventForObserver),
    GameEndedEventForBot(GameEndedEventForBot),
    GameEndedEventForObserver(GameEndedEventForObserver),
    GameAbortedEvent(GameAbortedEvent),
    GamePausedEventForObserver,
    GameResumedEventForObserver,

    RoundStartedEvent(RoundStartedEvent),
    RoundEndedEventForBot(RoundEndedEventForBot),
    RoundEndedEventForObserver(RoundEndedEventForObserver),

    ChangeTps(ChangeTps),
    TpsChangedEvent(TpsChangedEvent),

    BotDeathEvent(BotDeathEvent),
    BotHitBotEvent(BotHitBotEvent),
    BotHitWallEvent(BotHitWallEvent),
    BulletFiredEvent(BulletFiredEvent),
    BulletHitBotEvent(BulletHitBotEvent),
    BulletHitBulletEvent(BulletHitBulletEvent),
    BulletHitWallEvent(BulletHitWallEvent),
    HitByBulletEvent(HitByBulletEvent),
    ScannedBotEvent(ScannedBotEvent),
    SkippedTurnEvent,
    TickEventForBot(TickEventForBot),
    TickEventForObserver(TickEventForObserver),
    WonRoundEvent,
    TeamMessageEvent(TeamMessageEvent),

    StartGame(StartGame),
    StopGame,
    PauseGame,
    ResumeGame,
    NextTurn,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum BotEvent {
    BotDeathEvent(BotDeathEvent),
    BotHitBotEvent(BotHitBotEvent),
    BotHitWallEvent(BotHitWallEvent),
    BulletFiredEvent(BulletFiredEvent),
    BulletHitBotEvent(BulletHitBotEvent),
    BulletHitBulletEvent(BulletHitBulletEvent),
    BulletHitWallEvent(BulletHitWallEvent),
    HitByBulletEvent(HitByBulletEvent),
    ScannedBotEvent(ScannedBotEvent),
    SkippedTurnEvent(SkippedTurnEvent),
    TeamMessageEvent(TeamMessageEvent),
    TickEventForBot(TickEventForBot),
    TickEventForObserver(TickEventForObserver),
    WonRoundEvent(WonRoundEvent),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum WebsocketMessage {
    TickEventForBot(TickEventForBot),
    RoundStartedEvent(RoundStartedEvent),
    RoundEndedEventForBot(RoundEndedEventForBot),
    GameStartedEventForBot(GameStartedEventForBot),
    GameEndedEventForBot(GameEndedEventForBot),
    SkippedTurnEvent(SkippedTurnEvent),
    ServerHandshake(ServerHandshake),
    GameAbortedEvent(GameAbortedEvent),
}

impl TryFrom<MessageType> for WebsocketMessage {

    type Error = String;
    fn try_from(value: MessageType) -> Result<Self, Self::Error> {
        match value {
            MessageType::TickEventForBot(value) => { Ok(WebsocketMessage::TickEventForBot(value)) },
            MessageType::RoundStartedEvent(value) => { Ok(WebsocketMessage::RoundStartedEvent(value)) },
            MessageType::RoundEndedEventForBot(value) => { Ok(WebsocketMessage::RoundEndedEventForBot(value)) },
            MessageType::GameStartedEventForBot(value) => { Ok(WebsocketMessage::GameStartedEventForBot(value)) },
            MessageType::GameEndedEventForBot(value) => { Ok(WebsocketMessage::GameEndedEventForBot(value)) },
            MessageType::SkippedTurnEvent => { Ok(WebsocketMessage::SkippedTurnEvent) },
            MessageType::ServerHandshake(value) => { Ok(WebsocketMessage::ServerHandshake(value)) },
            MessageType::GameAbortedEvent(value) => { Ok(WebsocketMessage::GameAbortedEvent(value)) },
            _ => Err(format!("Unsupported WebSocket message type: {:?}", value))
        }
    }
}