# **PvP**

**Ping Versus Pong - The classical ping pong game in Rust and C.**

---

| Source | Description |
| -- | -- |
| c_engine | OpenGL wrapper with C code |
| pvp_game | [FUTURE WORK] Rust-based ping pong game for LLM using `c_engine` and `rust_engine` |
| rust_engine | Rust code to power `pvp_game` with `c_engine` dependency |
| rust_test_game | Rust code to apply `rust_engine` for Udacity submission |

## **1. Introduction**

This project is my submission for the Udacity course "Introduction to Rust".

### **1.1. The Submission**

The repo consists of a prepared C code with OpenGL dependency `starter` (renamed to `c_engine`) by the Udacity team. My contribution is the Rust library `rust_engine`, and the binary `rust_test_game` that applies `rust_engine` with Rust standard libraries for multithreading and Rust crate `reqwest` to wits its blocking Client API to perform the HTTP request and to block the current thread until the full response (or an error) arrives.

### **1.2. Future Work**

For future work, I will develop `pvp_game` which is essentially a ping pong for 2 players (the title of this repository Ping vs Pong) where player 1 controls its block movement with W (up) and S (down) and player 2 controls its block with UP arrow and DOWN arrow, on one keyboard for simplicity.

### **1.3. Project Structure**

The project structure:

```shell
PVP
├── c_engine
│   ├── c_output
│   │   ├── libopenglwrapper.so
│   │   ├── opengl_wrapper_lib.o
│   │   └── test_game_exe
│   ├── c_test_game
│   │   └── test_game.c
│   ├── LICENSE.txt
│   ├── Makefile
│   ├── opengl_wrapper_lib
│   │   ├── opengl_wrapper_lib.c
│   │   └── opengl_wrapper_lib.h
│   └── README.md
├── pvp_game
│   ├── Cargo.toml
│   └── src
│       └── main.rs
├── README.md
├── rust_engine
│   ├── build.rs
│   ├── Cargo.lock
│   ├── Cargo.toml
│   ├── src
│   │   ├── lib.rs
│   │   ├── sprite.rs
│   │   └── window.rs
│   └── tests
│       ├── integration_test.rs
│       └── unit_test.rs
└── rust_test_game
    ├── Cargo.lock
    ├── Cargo.toml
    ├── README.md
    └── src
        ├── command.rs
        ├── data.rs
        ├── main.rs
        └── networking.rs
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

## **4. Submission Checklist**

<table>
  <thead>
    <tr>
      <th>Delivery</th>
      <th>Tasks</th>
      <th>Description</th>
      <th>Status</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td rowspan="2">Proper setup and initial configuration</td>
      <td>c_test_game runs successfully</td>
      <td><a href="https://github.com/Maverick-Intelligence/PvP/blob/trunk/c_engine/c_test_game/test_game.c">test_game.c</a></td>
      <td>✅</td>
    </tr>
    <tr>
      <td>A library project for the game engine is created</td>
      <td><a href="https://github.com/Maverick-Intelligence/PvP/blob/trunk/rust_engine/src/lib.rs">lib.rs</a></td>
      <td>✅</td>
    </tr>
    <tr>
      <td rowspan="4">Compilation and linking of the C library and implementation of FFI bindings</td>
      <td>build.rs script is created and compiles the C library without errors</td>
      <td><a href="https://github.com/Maverick-Intelligence/PvP/blob/trunk/rust_engine/build.rs">build.rs</a></td>
      <td>✅</td>
    </tr>
    <tr>
      <td>extern block for C functions is created, referring to opengl_wrapper_lib/opengl_wrapper.h for the function signatures</td>
      <td><a href="https://github.com/Maverick-Intelligence/PvP/blob/trunk/rust_engine/src/window.rs">window.rs</a>, <a href="https://github.com/Maverick-Intelligence/PvP/blob/trunk/rust_engine/src/sprite.rs">sprite.rs</a></td>
      <td>✅</td>
    </tr>
    <tr>
      <td>Rust functions calling the C functions are implemented</td>
      <td><a href="https://github.com/Maverick-Intelligence/PvP/blob/trunk/rust_engine/src/window.rs">window.rs</a>, <a href="https://github.com/Maverick-Intelligence/PvP/blob/trunk/rust_engine/src/sprite.rs">sprite.rs</a></td>
      <td>✅</td>
    </tr>
    <tr>
      <td>Macro-annotated structs for the data types passed between the Rust and C code are created</td>
      <td><a href="https://github.com/Maverick-Intelligence/PvP/blob/trunk/rust_engine/src/sprite.rs">sprite.rs</a>, <a href="https://github.com/Maverick-Intelligence/PvP/blob/trunk/rust_engine/src/window.rs">window.rs</a></td>
      <td>✅</td>
    </tr>
    <tr>
      <td rowspan="5">Implementation and verification of tests</td>
      <td>test_simple_game_loop is implemented completely</td>
      <td><a href="https://github.com/Maverick-Intelligence/PvP/blob/trunk/rust_engine/tests/integration_test.rs">integration_test.rs</a></td>
      <td>✅</td>
    </tr>
    <tr>
      <td>test_sprite_rendering is implemented</td>
      <td><a href="https://github.com/Maverick-Intelligence/PvP/blob/trunk/rust_engine/tests/integration_test.rs">integration_test.rs</a></td>
      <td>✅</td>
    </tr>
    <tr>
      <td>test_screen_clearing is implemented</td>
      <td><a href="https://github.com/Maverick-Intelligence/PvP/blob/trunk/rust_engine/tests/integration_test.rs">integration_test.rs</a></td>
      <td>✅</td>
    </tr>
    <tr>
      <td>test_key_presses is implemented</td>
      <td><a href="https://github.com/Maverick-Intelligence/PvP/blob/trunk/rust_engine/tests/integration_test.rs">integration_test.rs</a></td>
      <td>✅</td>
    </tr>
    <tr>
      <td>test_sprite_position_update is implemented</td>
      <td><a href="https://github.com/Maverick-Intelligence/PvP/blob/trunk/rust_engine/tests/integration_test.rs">integration_test.rs</a></td>
      <td>✅</td>
    </tr>
    <tr>
      <td rowspan="4">Creating Declarative Macros</td>
      <td>start_window_and_game_loop! macro is implemented</td>
      <td><a href="https://github.com/Maverick-Intelligence/PvP/blob/trunk/rust_engine/src/lib.rs">lib.rs</a></td>
      <td>✅</td>
    </tr>
    <tr>
      <td>on_key_press! macro is implemented</td>
      <td><a href="https://github.com/Maverick-Intelligence/PvP/blob/trunk/rust_engine/src/lib.rs">lib.rs</a></td>
      <td>✅</td>
    </tr>
    <tr>
      <td>spawn_sprite! macro is implemented</td>
      <td><a href="https://github.com/Maverick-Intelligence/PvP/blob/trunk/rust_engine/src/lib.rs">lib.rs</a></td>
      <td>✅</td>
    </tr>
    <tr>
      <td>At least one additional macro is implemented</td>
      <td><a href="https://github.com/Maverick-Intelligence/PvP/blob/trunk/rust_engine/src/lib.rs">lib.rs</a></td>
      <td>✅</td>
    </tr>
    <tr>
      <td rowspan="4">Development of a simple game</td>
      <td>A new binary project is created (e.g., rust_test_game)</td>
      <td><a href="https://github.com/Maverick-Intelligence/PvP/blob/trunk/rust_test_game/src/main.rs">main.rs</a></td>
      <td>✅</td>
    </tr>
    <tr>
      <td>Game loop and key press listeners are implemented</td>
      <td><a href="https://github.com/Maverick-Intelligence/PvP/blob/trunk/rust_test_game/src/main.rs">main.rs</a></td>
      <td>✅</td>
    </tr>
    <tr>
      <td>HTTP request to fetch sprite data is correctly handled with multithreading</td>
      <td><a href="https://github.com/Maverick-Intelligence/PvP/blob/trunk/rust_test_game/src/networking.rs">networking.rs</a>, <a href="https://github.com/Maverick-Intelligence/PvP/blob/trunk/rust_test_game/src/main.rs">main.rs</a></td>
      <td>✅</td>
    </tr>
    <tr>
      <td>Proper modularization, error handling, and code cleanliness are maintained</td>
      <td><a href="https://github.com/Maverick-Intelligence/PvP/blob/trunk/rust_test_game/src/main.rs">main.rs</a>, <a href="https://github.com/Maverick-Intelligence/PvP/blob/trunk/rust_test_game/src/command.rs">command.rs</a>, <a href="https://github.com/Maverick-Intelligence/PvP/blob/trunk/rust_test_game/src/data.rs">data.rs</a>, <a href="https://github.com/Maverick-Intelligence/PvP/blob/trunk/rust_test_game/src/networking.rs">networking.rs</a></td>
      <td>✅</td>
    </tr>
  </tbody>
</table>

--- 

NOTE: I used my local LLM to support with the requirement engineering without **ANY CONTRIBUTION IN THE CODING ITSELF**.
