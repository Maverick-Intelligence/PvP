use std::error::Error;

use reqwest::blocking::Client;

use crate::command::{GameCommand, GameCommandReceiver};
use crate::data::{GameData, GameDataSender};

const SPRITE_ENDPOINT: &str =
    "https://get-random-sprite-data-dan-chiarlones-projects.vercel.app/api/handler";

pub fn run(
    commands: GameCommandReceiver,
    results: GameDataSender,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let client = Client::new();

    loop {
        match commands.recv()? {
            GameCommand::Quit => break,
            GameCommand::FetchGameData => match fetch_game_data(&client) {
                Ok(data) => {
                    if results.send(data).is_err() {
                        break;
                    }
                }
                Err(error) => eprintln!("failed to fetch sprite data: {error}"),
            },
        }
    }

    Ok(())
}

fn fetch_game_data(client: &Client) -> Result<GameData, reqwest::Error> {
    let data = client.get(SPRITE_ENDPOINT).send()?.json::<GameData>()?;
    Ok(data)
}
