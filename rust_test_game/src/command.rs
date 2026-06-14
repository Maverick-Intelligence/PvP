use std::sync::mpsc::{self, Receiver, RecvError, SendError, Sender};

pub enum GameCommand {
    FetchGameData,
    Quit,
}

pub struct GameCommandSender(Sender<GameCommand>);

impl GameCommandSender {
    pub fn send(&self, command: GameCommand) -> Result<(), SendError<GameCommand>> {
        self.0.send(command)
    }
}

pub struct GameCommandReceiver(Receiver<GameCommand>);

impl GameCommandReceiver {
    pub fn recv(&self) -> Result<GameCommand, RecvError> {
        self.0.recv()
    }
}

pub fn get_command_channel() -> (GameCommandSender, GameCommandReceiver) {
    let (sender, receiver) = mpsc::channel();
    (GameCommandSender(sender), GameCommandReceiver(receiver))
}
