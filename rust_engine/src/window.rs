use std::ffi::CString;
use std::marker::{PhantomData, PhantomPinned};
use std::os::raw::{c_char, c_int, c_uchar, c_uint};

pub const GLFW_PRESS: c_int = 1;
pub const GLFW_KEY_SPACE: c_int = 32;
pub const GLFW_KEY_RIGHT: c_int = 262;
pub const GLFW_KEY_LEFT: c_int = 263;
pub const GLFW_KEY_DOWN: c_int = 264;
pub const GLFW_KEY_UP: c_int = 265;
pub const GLFW_KEY_A: c_int = 65;
pub const GLFW_KEY_W: c_int = 87;
pub const GLFW_KEY_D: c_int = 68;
pub const GLFW_KEY_S: c_int = 83;

#[repr(C)]
pub struct GLFWwindow {
    _data: [usize; 0],
    _marker: PhantomData<(*mut usize, PhantomPinned)>,
}

unsafe extern "C" {
    fn create_game_window(title: *const c_char, width: c_uint, height: c_uint);

    fn update_game_window();

    fn clear_screen();

    fn window_should_close() -> c_uchar;

    fn get_key(window: *const GLFWwindow, key: c_int) -> c_int;

    fn get_window() -> *const GLFWwindow;
}

pub fn create_pvp_window(title: String, width: u32, height: u32) {
    let c_str = CString::new(title).expect("CString::new failed!");
    let title_as_c_ptr: *const c_char = c_str.as_ptr();

    unsafe { create_game_window(title_as_c_ptr, width, height) }
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
