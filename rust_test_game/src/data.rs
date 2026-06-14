use serde::Deserialize;

use std::sync::mpsc::{self, Receiver, SendError, Sender, TryRecvError};

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct GameData {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub r: i32,
    pub g: i32,
    pub b: i32,
}

pub struct GameDataSender(Sender<GameData>);

impl GameDataSender {
    pub fn send(&self, data: GameData) -> Result<(), SendError<GameData>> {
        self.0.send(data)
    }
}

pub struct GameDataReceiver(Receiver<GameData>);

impl GameDataReceiver {
    pub fn try_recv(&self) -> Result<GameData, TryRecvError> {
        self.0.try_recv()
    }
}

pub fn get_data_channel() -> (GameDataSender, GameDataReceiver) {
    let (sender, receiver) = mpsc::channel();
    (GameDataSender(sender), GameDataReceiver(receiver))
}
