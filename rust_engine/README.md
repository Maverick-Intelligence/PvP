# **Rust Engine**

`rust_engine` is the Rust game-engine library for **PvP**. It wraps the C/OpenGL
code in `c_engine` through FFI, exposes safe `Sprite` and window APIs, and ships a
set of declarative macros that drive a game loop.

## 1. What's Inside

| File | What it does |
| -- | -- |
| [build.rs](https://github.com/Maverick-Intelligence/PvP/blob/trunk/rust_engine/build.rs) | Build script: compiles the C engine (`opengl_wrapper_lib`) with `cc` and links GLFW + OpenGL via `pkg-config`. |
| [src/lib.rs](https://github.com/Maverick-Intelligence/PvP/blob/trunk/rust_engine/src/lib.rs) | Crate root: declares the modules and defines the declarative macros (`spawn_sprite!`, `move_sprite!`, `tick!`, `on_key_press!`, `change_sprite_color!`, `start_window_and_game_loop!`). |
| [src/sprite.rs](https://github.com/Maverick-Intelligence/PvP/blob/trunk/rust_engine/src/sprite.rs) | The `#[repr(C)]` `Sprite` struct, its C FFI bindings, and `impl Sprite` (`new`, `render`, `update_position`, `set_color`). |
| [src/window.rs](https://github.com/Maverick-Intelligence/PvP/blob/trunk/rust_engine/src/window.rs) | The opaque `GLFWwindow` type, GLFW key constants, the window C FFI bindings, and the safe window functions. |
| [tests/integration_test.rs](https://github.com/Maverick-Intelligence/PvP/blob/trunk/rust_engine/tests/integration_test.rs) | The 5 visual integration tests (window loop, sprite rendering, screen clearing, key presses, sprite movement). |
| [tests/unit_test.rs](https://github.com/Maverick-Intelligence/PvP/blob/trunk/rust_engine/tests/unit_test.rs) | Unit test for `Sprite` construction. |
| [Cargo.toml](https://github.com/Maverick-Intelligence/PvP/blob/trunk/rust_engine/Cargo.toml) | Crate manifest: build dependencies (`cc`, `pkg-config`) and the test targets. |

## 2. Tasks Completed In This Project

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
      <td>Proper setup and initial configuration</td>
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
  </tbody>
</table>
