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

#[test]
fn test_screen_clearing() {
    let duration_1 = std::time::Duration::from_millis(1000);
    let duration_2 = std::time::Duration::from_millis(10);
    let title = String::from("Test Screen Clearing");

    rust_engine::window::create_pvp_window(title, 800, 600);

    let sprite_red = rust_engine::sprite::create_pvp_sprite(0.0, 0.0, 80, 80, 255, 0, 0);
    let sprite_green = rust_engine::sprite::create_pvp_sprite(0.0, 0.0, 80, 80, 0, 255, 0);

    rust_engine::sprite::render_pvp_sprite(sprite_red);
    rust_engine::window::update_pvp_window();
    std::thread::sleep(duration_1);
    rust_engine::window::clear_pvp_window_screen();

    loop {
        rust_engine::sprite::render_pvp_sprite(sprite_green);
        rust_engine::window::update_pvp_window();
        if rust_engine::window::close_pvp_window() == 1 {
            break;
        }
        std::thread::sleep(duration_2);
    }

    assert_ne!(sprite_red, std::ptr::null_mut());
    assert_ne!(sprite_green, std::ptr::null_mut());
}

#[test]
fn test_key_presses() {
    let duration = std::time::Duration::from_millis(50);
    let title = String::from("Test Sprite Rendering");
    let mut is_key_space_pressed = false;
    let mut is_key_right_pressed = false;
    let mut is_key_left_pressed = false;
    let mut is_key_down_pressed = false;
    let mut is_key_up_pressed = false;
    let sprite_red = rust_engine::sprite::create_pvp_sprite(0.0, 0.0, 80, 80, 255, 0, 0);
    let sprite_green = rust_engine::sprite::create_pvp_sprite(0.0, 0.0, 80, 80, 0, 255, 0);
    let sprite_blue = rust_engine::sprite::create_pvp_sprite(0.0, 0.0, 80, 80, 0, 0, 255);
    let sprite_yellow = rust_engine::sprite::create_pvp_sprite(0.0, 0.0, 80, 80, 255, 255, 0);
    let sprite_violet = rust_engine::sprite::create_pvp_sprite(0.0, 0.0, 80, 80, 127, 0, 255);

    rust_engine::window::create_pvp_window(title, 800, 600);

    loop {
        rust_engine::window::update_pvp_window();
        if rust_engine::window::get_pvp_key(
            rust_engine::window::get_pvp_window(),
            rust_engine::window::GLFW_KEY_SPACE,
        ) == rust_engine::window::GLFW_PRESS
        {
            is_key_space_pressed = true;
            rust_engine::sprite::render_pvp_sprite(sprite_red);
            rust_engine::window::update_pvp_window();
        } else if rust_engine::window::get_pvp_key(
            rust_engine::window::get_pvp_window(),
            rust_engine::window::GLFW_KEY_RIGHT,
        ) == rust_engine::window::GLFW_PRESS
        {
            is_key_right_pressed = true;
            rust_engine::sprite::render_pvp_sprite(sprite_green);
            rust_engine::window::update_pvp_window();
        } else if rust_engine::window::get_pvp_key(
            rust_engine::window::get_pvp_window(),
            rust_engine::window::GLFW_KEY_LEFT,
        ) == rust_engine::window::GLFW_PRESS
        {
            is_key_left_pressed = true;
            rust_engine::sprite::render_pvp_sprite(sprite_blue);
            rust_engine::window::update_pvp_window();
        } else if rust_engine::window::get_pvp_key(
            rust_engine::window::get_pvp_window(),
            rust_engine::window::GLFW_KEY_UP,
        ) == rust_engine::window::GLFW_PRESS
        {
            is_key_up_pressed = true;
            rust_engine::sprite::render_pvp_sprite(sprite_yellow);
            rust_engine::window::update_pvp_window();
        } else if rust_engine::window::get_pvp_key(
            rust_engine::window::get_pvp_window(),
            rust_engine::window::GLFW_KEY_DOWN,
        ) == rust_engine::window::GLFW_PRESS
        {
            is_key_down_pressed = true;
            rust_engine::sprite::render_pvp_sprite(sprite_violet);
            rust_engine::window::update_pvp_window();
        }
        if rust_engine::window::close_pvp_window() == 1 {
            break;
        }
        std::thread::sleep(duration);
        rust_engine::window::clear_pvp_window_screen();
    }

    assert_eq!(is_key_space_pressed, true);
    assert_eq!(is_key_right_pressed, true);
    assert_eq!(is_key_left_pressed, true);
    assert_eq!(is_key_down_pressed, true);
    assert_eq!(is_key_up_pressed, true);
}
