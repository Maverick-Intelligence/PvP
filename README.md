# **PvP**

**Ping Versus Pong - The classical ping pong game in Rust and C.**

---

| Source | Description |
| -- | -- |
| c_engine | OpenGL wrapper with C code |
| pvp_game | Rust-based ping pong game for LLM using `c_engine` and `rust_engine` |
| rust_engine | Rust code to power `pvp_game` with `c_engine` dependency |

## **1. Introduction**

This project is my submission for the Udacity course "Introduction to Rust". The idea is to have a framework where people can just use 2 of their favourite LLMs with a zero-shot role-base system prompt to turn them into pingpong players fighting each other "Agent Ping vs Agent Pong".

The repo consists of a prepared C code with OpenGL dependency `starter` (renamed to `c_engine`) by the Udacity team. My contribution is the Rust library `ping`, and the binary `pong`. I used my local LLM to support with the requirement engineering without **ANY CONTRIBUTION IN THE CODING ITSELF**.

The project structure:
```shell
├── c_engine
│   ├── c_output
│   │   ├── .gitkeep
│   ├── c_test_game
│   │   └── test_game.c
│   ├── LICENSE.txt
│   ├── Makefile
│   ├── opengl_wrapper_lib
│   │   ├── opengl_wrapper_lib.c
│   │   └── opengl_wrapper_lib.h
│   └── README.md
├── pvp_game
│   ├── Cargo.toml
│   ├── README.md
│   └── src
│       └── main.rs
├── README.md
└── rust_engine
    ├── build.rs
    ├── Cargo.toml
    ├── README.md
    ├── src
    │   └── lib.rs
    └── tests
        ├── unit_test.rs
        └── integration_test.rs
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

🤖 LLM Agent: Writing a data processing pipeline for LLM and LLM tools to play a game.
