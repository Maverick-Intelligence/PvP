pub mod sprite;
pub mod window;

#[macro_export]
macro_rules! spawn_sprite {
<<<<<<< HEAD
    ($x:expr, $y:expr, $w:expr, $h:expr, $r:expr, $g:expr, $b:expr) => {{
        let mut tmp_sprite = $crate::sprite::Sprite::new($x, $y, $w, $h, $r, $g, $b);
        tmp_sprite.render();
        tmp_sprite
    }};
=======
    ($x:expr, $y:expr, $w:expr, $h:expr, $r:expr, $g:expr, $b:expr) => {
        let tmp_sprite = $crate::sprite::create_pvp_sprite($x, $y, $w, $h, $r, $g, $b);
        $crate::sprite::render_pvp_sprite(tmp_sprite);
    };
>>>>>>> 88f77e3 (refactor(integration-test): finalise test_key_presses with macro loop)
}

#[macro_export]
macro_rules! move_sprite {
    (clear, $sprite:expr, $x:expr, $y:expr) => {{
        $crate::window::clear_pvp_window_screen();
<<<<<<< HEAD
        $sprite.update_position($x, $y);
        $sprite.render();
    }};
    (no_clear, $sprite:expr, $x:expr, $y:expr) => {{
        $sprite.update_postition($x, $y);
        $sprite.render();
=======
        $crate::sprite::update_pvp_sprite_position($sprite, $x, $y);
        $crate::sprite::render_pvp_sprite($sprite);
    }};
    (no_clear, $sprite:expr, $x:expr, $y:expr) => {{
        $crate::sprite::update_pvp_sprite_position($sprite, $x, $y);
        $crate::sprite::render_pvp_sprite($sprite);
>>>>>>> 88f77e3 (refactor(integration-test): finalise test_key_presses with macro loop)
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
    ($sprite:expr, $r:expr, $g:expr, $b:expr) => {{
<<<<<<< HEAD
        $sprite.set_color($r, $g, $b);
        $sprite.render();
=======
        let tmp_sprite = $crate::sprite::pvp_sprite_ref($sprite);
        $crate::spawn_sprite!(
            tmp_sprite.x,
            tmp_sprite.y,
            tmp_sprite.width,
            tmp_sprite.height,
            $r,
            $g,
            $b
        );
>>>>>>> 88f77e3 (refactor(integration-test): finalise test_key_presses with macro loop)
    }};
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
<<<<<<< HEAD
            $crate::tick!(10);
=======
            $crate::tick!();
>>>>>>> 88f77e3 (refactor(integration-test): finalise test_key_presses with macro loop)
            $crate::window::clear_pvp_window_screen();
        }
    }};
}
