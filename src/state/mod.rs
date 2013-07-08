/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: state/mod.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      An aggregator of state items.
*/

/* Director */
pub use self::director::Director;
pub use self::director::State;

pub use self::game::Game;
pub use self::game_renderer::Game_Renderer;
pub use self::bsp_renderer::BSP_Renderer;
pub use self::console::Console;

mod director;

#[path = "game/game.rs"]
mod game;
#[path = "game_renderer/game_renderer.rs"]
mod game_renderer;
#[path = "bsp_renderer/bsp_renderer.rs"]
mod bsp_renderer;
#[path = "console/console.rs"]
mod console;

