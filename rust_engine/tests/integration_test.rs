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
