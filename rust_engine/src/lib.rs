pub mod sprite;
pub mod window;

#[macro_export]
macro_rules! spawn_sprite {
    ($x:expr, $y:expr, $w:expr, $h:expr, $r:expr, $g:expr, $b:expr) => {{
        let mut tmp_sprite = $crate::sprite::Sprite::new($x, $y, $w, $h, $r, $g, $b);
        tmp_sprite.render();
        tmp_sprite
    }};
}

#[macro_export]
macro_rules! move_sprite {
    (clear, $sprite:expr, $x:expr, $y:expr) => {{
        $crate::window::clear_pvp_window_screen();
        $sprite.update_position($x, $y);
        $sprite.render();
    }};
    (no_clear, $sprite:expr, $x:expr, $y:expr) => {{
        $sprite.update_postition($x, $y);
        $sprite.render();
    }};
}

#[macro_export]
macro_rules! tick {
    ($ms:expr) => {
        let tmp_duration_in_ms = std::time::Duration::from_millis($ms);
        $crate::window::update_pvp_window();
        std::thread::sleep(tmp_duration_in_ms);
    };
}

#[macro_export]
macro_rules! on_key_press {
    ($( $key:expr => $action:expr ),+ $(,)?) => {
        $(
            if $crate::window::get_pvp_key($crate::window::get_pvp_window(), $key)
                == $crate::window::GLFW_PRESS
            {
                $action;
                $crate::window::update_pvp_window();
            }
        )+
    };
}

#[macro_export]
macro_rules! change_sprite_color {
    ($sprite:expr, $r:expr, $g:expr, $b:expr) => {
        $sprite.set_color($r, $g, $b);
        $sprite.render();
    };
}

#[macro_export]
macro_rules! start_window_and_game_loop {
    ($title:expr, $w:expr, $h:expr, $($action:tt)*) => {{
        $crate::window::create_pvp_window($title, $w, $h);

        loop {
            $($action)*

            if $crate::window::close_pvp_window() == 1 {
                break;
            }
            $crate::tick!(10);
            $crate::window::clear_pvp_window_screen();
        }
    }};
}
