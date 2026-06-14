# **Rust Test Game**

## 1. Criteria	Submission Requirements

<table>
  <thead>
    <tr>
      <th>Delivery</th>
      <th>Tasks</th>
      <th>Status</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td rowspan="4">Development of a simple game</td>
      <td>A new binary project is created (e.g., rust_test_game)</td>
      <td>✅</td>
    </tr>
    <tr>
      <td>Game loop and key press listeners are implemented</td>
      <td>✅</td>
    </tr>
    <tr>
      <td>HTTP request to fetch sprite data is correctly handled with multithreading</td>
      <td>✅</td>
    </tr>
    <tr>
      <td>Proper modularization, error handling, and code cleanliness are maintained</td>
      <td>✅</td>
    </tr>
  </tbody>
</table>

# 2. Documentation

`rust_test_game` is a binary crate that drives the local `rust_engine` and fetches
random sprites from a slow HTTP endpoint without ever stalling the game loop. A
dedicated networking thread performs the blocking request, while the main thread
keeps rendering and picks up results via channels.

| File | Summary | Key code |
| -- | -- | -- |
| [`src/main.rs`](https://github.com/Maverick-Intelligence/PvP/blob/trunk/rust_test_game/src/main.rs) | Entry point. Creates the two channels, spawns the networking thread, runs `start_window_and_game_loop!`, requests a sprite on each `Space` press, receives finished sprites with `try_recv` and spawns them, and on quit sends `Quit` then joins the thread. | `main()` |
| [`src/command.rs`](https://github.com/Maverick-Intelligence/PvP/blob/trunk/rust_test_game/src/command.rs) | Main → networking channel: the command messages and their wrapped sender/receiver. | `GameCommand`, `GameCommandSender`, `GameCommandReceiver`, `get_command_channel()` |
| [`src/data.rs`](https://github.com/Maverick-Intelligence/PvP/blob/trunk/rust_test_game/src/data.rs) | Networking → main channel: the serde-deserialized sprite struct and its wrapped sender/receiver. | `GameData`, `GameDataSender`, `GameDataReceiver`, `get_data_channel()` |
| [`src/networking.rs`](https://github.com/Maverick-Intelligence/PvP/blob/trunk/rust_test_game/src/networking.rs) | The networking thread: listens for commands and performs the blocking HTTP fetch off the game loop. | `run()`, `fetch_sprite_data()` |
| [`Cargo.toml`](https://github.com/Maverick-Intelligence/PvP/blob/trunk/rust_test_game/Cargo.toml) | Declares the local engine path dependency plus `reqwest` (blocking/json/rustls) and `serde`. | `[dependencies]` |
