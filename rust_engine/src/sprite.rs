#[repr(C)]
pub struct Sprite {
    pub width: u32,
    pub height: u32,
    pub color: [u8; 3],
    pub x: f32,
    pub y: f32,
}

unsafe extern "C" {
    fn create_sprite(
        x: std::os::raw::c_float,
        y: std::os::raw::c_float,
        width: std::os::raw::c_uint,
        height: std::os::raw::c_uint,
        r: u8,
        g: u8,
        b: u8,
    ) -> *const Sprite;

    fn render_sprite(sprite: *const Sprite);

    fn update_sprite_position(
        sprite: *const Sprite,
        x: std::os::raw::c_float,
        y: std::os::raw::c_float,
    );
}

pub fn create_pvp_sprite(
    x: f32,
    y: f32,
    width: u32,
    height: u32,
    r: u8,
    g: u8,
    b: u8,
) -> *const Sprite {
    unsafe { create_sprite(x, y, width, height, r, g, b) }
}

pub fn render_pvp_sprite(sprite: *const Sprite) {
    unsafe { render_sprite(sprite) }
}

pub fn update_pvp_sprite_position(sprite: *const Sprite, x: f32, y: f32) {
    unsafe { update_sprite_position(sprite, x, y) }
}

pub fn pvp_sprite_ref<'a>(sprite: *const Sprite) -> &'a Sprite {
    unsafe { &*sprite }
}
