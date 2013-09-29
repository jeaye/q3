/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: state/mod.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      An aggregator of state items.
*/

#[link(name = "state", vers = "0.2")];
#[crate_type = "lib"];

extern mod extra;
extern mod opengles;
extern mod glfw;

extern mod log;
extern mod console;
extern mod math;
extern mod gl;
extern mod ui;
extern mod obj;

pub use self::director::{ State, Director, Deferred };
pub use self::game::Game;
pub use self::game_renderer::Game_Renderer;
pub use self::map_renderer::Map_Renderer;
pub use self::bsp_renderer::BSP_Renderer;
pub use self::console_renderer::Console_Renderer;

pub mod director;
#[path = "game/game.rs"]
pub mod game;
#[path = "game/game_renderer.rs"]
pub mod game_renderer;
#[path = "game/map_renderer.rs"]
pub mod map_renderer;
#[path = "game/bsp_renderer.rs"]
pub mod bsp_renderer;
#[path = "console/console_renderer.rs"]
pub mod console_renderer;
#[path = "camera/camera.rs"]
pub mod camera;

