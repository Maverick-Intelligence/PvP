use std::os::raw::{c_float, c_int, c_uint};

#[repr(C)]
pub struct Sprite {
    pub width: u32,
    pub height: u32,
    pub color: [i32; 3],
    pub x: f32,
    pub y: f32,
}

unsafe extern "C" {
    fn create_sprite(
        x: c_float,
        y: c_float,
        width: c_uint,
        height: c_uint,
        r: c_int,
        g: c_int,
        b: c_int,
    ) -> *const Sprite;

    fn render_sprite(sprite: *const Sprite);

    fn update_sprite_position(sprite: *mut Sprite, x: c_float, y: c_float);
}

impl Sprite {
    pub fn new(x: f32, y: f32, width: u32, height: u32, r: i32, g: i32, b: i32) -> Self {
        unsafe { std::ptr::read(create_sprite(x, y, width, height, r, g, b)) }
    }

    pub fn render(&self) {
        unsafe { render_sprite(self) };
    }

    pub fn update_position(&mut self, x: f32, y: f32) {
        unsafe { update_sprite_position(self, x, y) };
    }

    pub fn set_color(&mut self, r: i32, g: i32, b: i32) {
        self.color = [r, g, b];
    }
}

pub fn pvp_sprite_ref<'a>(sprite: *const Sprite) -> &'a Sprite {
    unsafe { &*sprite }
}
