extern crate rust_engine;

#[test]
fn test_create_sprite() {
    let sprite = rust_engine::sprite::Sprite {
        width: 800,
        height: 600,
        color: [150, 150, 150],
        x: 10.0,
        y: 10.0,
    };

    assert_eq!(sprite.x, 10.0);
    assert_eq!(sprite.y, 10.0);
}
