use std::ffi::CString;
use std::marker::{PhantomData, PhantomPinned};
use std::os::raw::c_char;

#[repr(C)]
pub struct GLFWwindow {
    _data: [usize; 0],
    _marker: PhantomData<(*mut usize, PhantomPinned)>,
}

#[repr(C)]
pub struct Sprite {
    pub width: u32,
    pub height: u32,
    pub color: [u8; 3],
    pub x: *mut f32,
    pub y: *mut f32,
}

unsafe extern "C" {
    fn create_game_window(title: *const c_char, width: u32, height: u32);

    fn create_sprite(x: f32, y: f32, width: u32, height: u32, r: u8, g: u8, b: u8)
    -> *const Sprite;

    fn render_sprite(sprite: *const Sprite);

    fn update_sprite_position(sprite: *const Sprite, x: f32, y: f32);

    fn update_game_window();

    fn clear_screen();

    fn window_should_close() -> u8;

    fn get_key(window: *const GLFWwindow, key: i32) -> i32;

    fn get_window() -> *const GLFWwindow;
}

pub fn create_pvp_window(title: String, width: u32, height: u32) {
    let c_str = CString::new(title).expect("CString::new failed!");
    let title_as_c_ptr: *const c_char = c_str.as_ptr();

    unsafe { create_game_window(title_as_c_ptr, width, height) }
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

pub fn update_pvp_window() {
    unsafe { update_game_window() }
}

pub fn clear_pvp_window_screen() {
    unsafe { clear_screen() }
}

pub fn close_pvp_window() -> u8 {
    unsafe { window_should_close() }
}

pub fn get_pvp_key(window: *const GLFWwindow, key: i32) -> i32 {
    unsafe { get_key(window, key) }
}

pub fn get_pvp_window() -> *const GLFWwindow {
    unsafe { get_window() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_game_loop() {
        let title = String::from("PvP");
        create_pvp_window(title, 800, 600);
        let sprite = create_pvp_sprite(0.0, 0.0, 50, 50, 150, 150, 150);

        while close_pvp_window() != 1 {
            let mut counter = 0;
            for i in 1..=100 {
                if i == 1 {
                    render_pvp_sprite(sprite);
                } else if i > 1 {
                    update_pvp_sprite_position(sprite, i as f32 + 100.0, i as f32 + 100.0);
                    render_pvp_sprite(sprite);
                }
                counter = counter + 1;
                update_pvp_window();
                clear_pvp_window_screen();
            }
        }
        assert_eq!(30, 30);
    }
}
