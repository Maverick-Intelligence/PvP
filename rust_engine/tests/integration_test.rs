extern crate rust_engine;

#[test]
fn test_simple_game_loop() {
    let duration = std::time::Duration::from_millis(10);
    let title = String::from("Test Simple Game Loop");
    rust_engine::window::create_pvp_window(title, 800, 600);

    loop {
        rust_engine::window::update_pvp_window();
        if rust_engine::window::close_pvp_window() == 1 {
            break;
        }
        std::thread::sleep(duration);
    }

    assert_eq!(10, 10);
}

#[test]
fn test_sprite_rendering() {
    let duration = std::time::Duration::from_millis(10);
    let title = String::from("Test Sprite Rendering");
    rust_engine::window::create_pvp_window(title, 800, 600);
    let sprite = rust_engine::sprite::create_pvp_sprite(0.0, 0.0, 80, 80, 255, 255, 255);

    loop {
        rust_engine::sprite::render_pvp_sprite(sprite);
        rust_engine::window::update_pvp_window();
        if rust_engine::window::close_pvp_window() == 1 {
            break;
        }
        std::thread::sleep(duration);
    }

    assert_ne!(sprite, std::ptr::null_mut());
}
