mod command;
mod data;
mod networking;

use std::error::Error;
use std::thread::spawn;

use rust_engine::sprite::Sprite;

use command::{GameCommand, get_command_channel};
use data::get_data_channel;

fn main() -> Result<(), Box<dyn Error>> {
    let title = String::from("Udacity - Introduction to Rust Submission | Rust Test Game");
    let (command_sender, command_receiver) = get_command_channel();
    let (data_sender, data_receiver) = get_data_channel();
    let networking_thread = spawn(move || networking::run(command_receiver, data_sender));
    let mut sprites: Vec<Sprite> = Vec::new();

    rust_engine::start_window_and_game_loop!(
        title,
        800,
        600,
        rust_engine::on_key_press! {
            rust_engine::window::GLFW_KEY_SPACE => {
                command_sender.send(GameCommand::FetchGameData)?;
            }
        };

        if let Ok(data) = data_receiver.try_recv() {
            sprites.push(
                rust_engine::spawn_sprite!(
                    data.x as f32,
                    data.y as f32,
                    data.width,
                    data.height,
                    data.r,
                    data.g,
                    data.b
                )
            )
        };

        for sprite in &sprites {
            sprite.render();
        }
    );

    command_sender.send(GameCommand::Quit)?;

    match networking_thread.join() {
        Ok(Ok(())) => {}
        Ok(Err(error)) => eprintln!("Networking thread error: {error}"),
        Err(_) => eprintln!("Networking thread panicked"),
    }

    Ok(())
}
