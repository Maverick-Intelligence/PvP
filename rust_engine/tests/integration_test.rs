extern crate rust_engine;

#[test]
fn test_simple_game_loop() {
    let title = String::from("PvP");
    rust_engine::window::create_pvp_window(title, 800, 600);
    let sprite = rust_engine::sprite::create_pvp_sprite(0.0, 0.0, 50, 50, 150, 150, 150);
    let mut loop_counter = 0;
    let total_loop = 100;

    for i in 1..=total_loop {
        if i == 1 {
            rust_engine::sprite::render_pvp_sprite(sprite);
        } else if i > 1 {
            rust_engine::sprite::update_pvp_sprite_position(
                sprite,
                i as f32 + 100.0,
                i as f32 + 100.0,
            );
            rust_engine::sprite::render_pvp_sprite(sprite);
        }
        loop_counter += 1;
        rust_engine::window::update_pvp_window();
        rust_engine::window::clear_pvp_window_screen();
        if i == total_loop {
            rust_engine::window::close_pvp_window();
        }
    }
    assert_eq!(loop_counter, total_loop);
}
