# **PvP**

**Ping Versus Pong - The classical ping pong game in Rust and C.**

---

| Source | Description |
| -- | -- |
| c_engine | OpenGL wrapper with C code |
| pvp_game | Rust-based ping pong game for LLM using `c_engine` and `rust_engine` |
| rust_engine | Rust code to power `pvp_game` with `c_engine` dependency |

## **1. Introduction**

This project is my submission for the Udacity course "Introduction to Rust". The idea came from the example of the Udacity task, where 2 players can play ping pong against each other.

The repo consists of a prepared C code with OpenGL dependency `starter` (renamed to `c_engine`) by the Udacity team. My contribution is the Rust library `rust_engine`, and the binary `pvp_game`. I used my local LLM to support with the requirement engineering without **ANY CONTRIBUTION IN THE CODING ITSELF**

The project structure:
```shell
PVP
├── c_engine
│   ├── c_output
│   │   ├── .gitkeep
│   ├── c_test_game
│   │   └── test_game.c
│   ├── opengl_wrapper_lib
│   │   ├── opengl_wrapper_lib.c
│   │   └── opengl_wrapper_lib.h
│   ├── LICENSE.txt
│   ├── Makefile
│   └── README.md
├── pvp_game
│   ├── src
│       └── main.rs
│   └── Cargo.toml
├── README.md
├── rust_engine
│   ├── src
│   │   ├── lib.rs
│   │   ├── sprite.rs
│   │   └── window.rs
│   ├── tests
│   │    ├── integration_test.rs
│   │    └── unit_test.rs
│   ├── build.rs
│   └── Cargo.toml
├── .gitignore
└── README.md
```

## **2. Goals**
This project aims to assess the following skills developed during the course:

🧠 General Rust Knowledge: Mastery of ownership rules, lifetimes, and other fundamental Rust concepts.

🧱 Modularized Code: Ability to write clean, modularized code using Rust's mod keyword.

⛓️‍💥 Error Handling: Effective use of structures like Result for error handling.

🧪 Testing: Implementation of unit tests and integration tests to ensure code correctness.

🚚 Cargo Utilization: Proficiency in using cargo features such as cargo test, cargo fmt, cargo clippy, and integrating Rust crates.

🌌 Macro Creation: Development of declarative macros to simplify and automate repetitive tasks.

🎭 Multithreaded: Implementation of multi-threading using Rust’s thread, message passing, and sync primitives (Arc and Mutex).

🧩 C Interoperability: Interfacing with C code from Rust, including compilation and binding creation.

## **3. Setup Guide**

### 3.1. Clone Source Code

* `git clone https://github.com/Maverick-Intelligence/PvP.git`

### 3.2. Running Test

To run the test:

```shell
cd c_engine
make test-rust
```

Then follow the below guide for 5 main integration tests from both `Sprite` and `Window` objects:

* `test_simple_game_loop()` -> Popping up a window, and tester can close the window instantly by clicking the x (close symbol).
* `test_sprite_rendering()` -> Popping up a window with a squared sprite on it. Tester can instantly close the window.
* `test_screen_clearing()` -> Popping up a window with a squared sprite on it that after 1 second changes colour, and then tester can close the window.
* `test_key_presses()` ->
  * Popping a window
  * Tester must click the following key so the test passes:
    * Space
    * Left Arrow
    * Up Arrow
    * Right Arrow
    * Down Arrow
    * A
    * W
    * D
    * S
* `test_sprite_position_update()` -> Popping up a window with a squared sprite going from up left to down right of the window. Tester can close the window after the sprite goes out of the window.
