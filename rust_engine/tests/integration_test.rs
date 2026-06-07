extern crate rust_engine;

#[test]
fn test_simple_game_loop() {
    let title = String::from("Test Simple Game Loop");

    rust_engine::start_window_and_game_loop!(title, 800, 600, rust_engine::window::update_pvp_window(););
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
    let sprite_pos_x = 0.0;
    let sprite_pos_y = 0.0;
    let sprite_dim = 80;
    let sprite_rgb = 0;
    let sprite = rust_engine::sprite::create_pvp_sprite(
        sprite_pos_x,
        sprite_pos_y,
        sprite_dim,
        sprite_dim,
        sprite_rgb,
        sprite_rgb,
        sprite_rgb,
    );

    rust_engine::start_window_and_game_loop!(
        title,
        800,
        600,
        rust_engine::on_key_press! {
            rust_engine::window::GLFW_KEY_SPACE => {
                is_key_space_pressed = true;
                rust_engine::change_sprite_color!(
                    sprite,
                    255,
                    0,
                    0
                );
            },
            rust_engine::window::GLFW_KEY_RIGHT  => {
                is_key_right_pressed = true;
                rust_engine::change_sprite_color!(
                    sprite,
                    0,
                    255,
                    0
                );
            },
            rust_engine::window::GLFW_KEY_LEFT => {
                is_key_left_pressed = true;
                rust_engine::change_sprite_color!(
                    sprite,
                    0,
                    0,
                    255
                );
            },
            rust_engine::window::GLFW_KEY_UP => {
                is_key_up_pressed = true;
                rust_engine::change_sprite_color!(
                    sprite,
                    255,
                    255,
                    0
                );
            },
            rust_engine::window::GLFW_KEY_DOWN  => {
                is_key_down_pressed = true;
                rust_engine::change_sprite_color!(
                    sprite,
                    127,
                    0,
                    255
                );
            },
            rust_engine::window::GLFW_KEY_W  => {
                is_key_w_pressed = true;
                rust_engine::change_sprite_color!(
                    sprite,
                    255,
                    0,
                    127
                );
            },
            rust_engine::window::GLFW_KEY_A => {
                is_key_a_pressed = true;
                rust_engine::change_sprite_color!(
                    sprite,
                    0,
                    255,
                    255
                );
            },
            rust_engine::window::GLFW_KEY_S => {
                is_key_s_pressed = true;
                rust_engine::change_sprite_color!(
                    sprite,
                    150,
                    75,
                    0
                );
            },
            rust_engine::window::GLFW_KEY_D  => {
                is_key_d_pressed = true;
                rust_engine::change_sprite_color!(
                    sprite,
                    169,
                    169,
                    169
                );
            }
        };
    );

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
    let title = String::from("Test Sprite Position Update");
    let mut counter_sprite_position = 1;

    rust_engine::start_window_and_game_loop!(
        title,
        800,
        600,
        let sprite = rust_engine::spawn_sprite!(0.0, 0.0, 80, 80, 255, 255, 255);
        rust_engine::move_sprite!(
            clear,
            sprite,
            10.0 * counter_sprite_position as f32,
            10.0 * counter_sprite_position as f32
        );
        rust_engine::window::update_pvp_window();
        counter_sprite_position += 1;
    );

    assert!(counter_sprite_position > 70);
}
