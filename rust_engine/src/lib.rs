pub mod sprite;
pub mod window;

#[macro_export]
macro_rules! spawn_sprite {
    ($x:ident, $y:ident, $w:ident, $h:ident, $r:ident, $g:ident, $b:ident) => {
        let tmp_sprite = $crate::sprite::create_pvp_sprite($x, $y, $w, $h, $r, $g, $b);
        $crate::sprite::render_pvp_sprite(tmp_sprite);
        tmp_sprite
    };
}

#[macro_export]
macro_rules! move_sprite {
    (1, ($sprite:ident, $x:ident, $y: ident)) => {
        $crate::window::clear_pvp_window_screen();
        $crate::sprite::update_pvp_sprite_position($sprite, $x, $x);
        $crate::sprite::render_pvp_sprite($sprite);
        $crate::window::update_pvp_window();
    };
    (2, ($sprite:ident, $x:ident, $y: ident)) => {
        $crate::sprite::update_pvp_sprite_position($sprite, $x, $x);
        $crate::sprite::render_pvp_sprite($sprite);
        $crate::window::update_pvp_window();
        $crate::window::clear_pvp_window_screen();
    };
}

#[macro_export]
macro_rules! tick {
    () => {
        let tmp_duration_in_ms = std::time::Duration::from_millis(30);
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
    ($sprite:ident, $r:ident, $g:ident, $b:ident) => {
        let tmp_sprite = spawn_sprite!(
            $sprite.x.clone(),
            $sprite.y.clone(),
            $sprite.width.clone(),
            $sprite.height.clone(),
            $r,
            $g,
            $b
        );
        tmp_sprite;
    };
}

#[macro_export]
macro_rules! start_window_and_game_loop {
    ($title:expr, $w:expr, $h:expr, $action:expr) => {{
        $crate::window::create_pvp_window($title, $w, $h);

        loop {
            $action;
            if $crate::window::close_pvp_window() == 1 {
                break;
            }
            $crate::tick!();
        }
    }};
}
