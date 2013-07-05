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

mod director;

#[path = "game/game.rs"]
mod game;
#[path = "game_renderer/game_renderer.rs"]
mod game_renderer;

