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
    let title = String::from("Test Key Presses");
    let mut is_key_space_pressed = false;
    let mut is_key_right_pressed = false;
    let mut is_key_left_pressed = false;
    let mut is_key_down_pressed = false;
    let mut is_key_up_pressed = false;
    let mut is_key_a_pressed = false;
    let mut is_key_w_pressed = false;
    let mut is_key_d_pressed = false;
    let mut is_key_s_pressed = false;
    let sprite_red = rust_engine::sprite::create_pvp_sprite(0.0, 0.0, 80, 80, 255, 0, 0);
    let sprite_green = rust_engine::sprite::create_pvp_sprite(0.0, 0.0, 80, 80, 0, 255, 0);
    let sprite_blue = rust_engine::sprite::create_pvp_sprite(0.0, 0.0, 80, 80, 0, 0, 255);
    let sprite_yellow = rust_engine::sprite::create_pvp_sprite(0.0, 0.0, 80, 80, 255, 255, 0);
    let sprite_violet = rust_engine::sprite::create_pvp_sprite(0.0, 0.0, 80, 80, 127, 0, 255);
    let sprite_pink = rust_engine::sprite::create_pvp_sprite(0.0, 0.0, 80, 80, 255, 0, 127);
    let sprite_cyan = rust_engine::sprite::create_pvp_sprite(0.0, 0.0, 80, 80, 0, 255, 255);
    let sprite_brown = rust_engine::sprite::create_pvp_sprite(0.0, 0.0, 80, 80, 150, 75, 0);
    let sprite_gray = rust_engine::sprite::create_pvp_sprite(0.0, 0.0, 80, 80, 169, 169, 169);

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
        } else if rust_engine::window::get_pvp_key(
            rust_engine::window::get_pvp_window(),
            rust_engine::window::GLFW_KEY_A,
        ) == rust_engine::window::GLFW_PRESS
        {
            is_key_a_pressed = true;
            rust_engine::sprite::render_pvp_sprite(sprite_pink);
            rust_engine::window::update_pvp_window();
        } else if rust_engine::window::get_pvp_key(
            rust_engine::window::get_pvp_window(),
            rust_engine::window::GLFW_KEY_W,
        ) == rust_engine::window::GLFW_PRESS
        {
            is_key_w_pressed = true;
            rust_engine::sprite::render_pvp_sprite(sprite_cyan);
            rust_engine::window::update_pvp_window();
        } else if rust_engine::window::get_pvp_key(
            rust_engine::window::get_pvp_window(),
            rust_engine::window::GLFW_KEY_D,
        ) == rust_engine::window::GLFW_PRESS
        {
            is_key_d_pressed = true;
            rust_engine::sprite::render_pvp_sprite(sprite_brown);
            rust_engine::window::update_pvp_window();
        } else if rust_engine::window::get_pvp_key(
            rust_engine::window::get_pvp_window(),
            rust_engine::window::GLFW_KEY_S,
        ) == rust_engine::window::GLFW_PRESS
        {
            is_key_s_pressed = true;
            rust_engine::sprite::render_pvp_sprite(sprite_gray);
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
    assert_eq!(is_key_a_pressed, true);
    assert_eq!(is_key_w_pressed, true);
    assert_eq!(is_key_d_pressed, true);
    assert_eq!(is_key_s_pressed, true);
}

#[test]
fn test_sprite_position_update() {
    let duration = std::time::Duration::from_millis(50);
    let title = String::from("Test Sprite Position Update");
    let sprite = rust_engine::sprite::create_pvp_sprite(0.0, 0.0, 80, 80, 255, 255, 255);
    let mut counter_sprite_position = 1;

    rust_engine::window::create_pvp_window(title, 800, 600);
    rust_engine::sprite::render_pvp_sprite(sprite);
    rust_engine::window::update_pvp_window();
    rust_engine::window::clear_pvp_window_screen();

    loop {
        rust_engine::sprite::render_pvp_sprite(sprite);
        rust_engine::sprite::update_pvp_sprite_position(
            sprite,
            10.0 * counter_sprite_position as f32,
            10.0 * counter_sprite_position as f32,
        );
        rust_engine::window::update_pvp_window();

        if counter_sprite_position > 70 && rust_engine::window::close_pvp_window() == 1 {
            break;
        }

        std::thread::sleep(duration);
        rust_engine::window::clear_pvp_window_screen();
        counter_sprite_position += 1;
    }

    assert_ne!(sprite, std::ptr::null_mut());
    assert!(counter_sprite_position > 70);
}
