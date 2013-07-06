/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: state/game/game.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      The main game state, shared between
      client and server; it loads and
      voxelizes a BSP map, handles
      players, and manages the game from
      a user's perspective. No rendering
      is included.
*/

use BSP_Map = bsp::Map;
use Voxel_Map = voxel::Map;
use super::State;

pub struct Game
{
  bsp_map: BSP_Map,
  voxel_map: @mut Voxel_Map,
  name: ~str,
}

impl Game
{
  pub fn new() -> @mut Game
  {
    let map = BSP_Map::new("data/maps/q3ctf1.bsp");
    let game = @mut Game
    {
      voxel_map: Voxel_Map::new(map.tris, 300),
      bsp_map: map,
      name: ~"Default",
    };

    game
  }
}

impl State for Game
{
  pub fn load(&mut self)
  {
    debug!("Loading game state.");
  }
  pub fn unload(&mut self)
  {
    debug!("Unloading game state.");
  }

  pub fn update(&mut self, delta: f32) -> bool /* dt is in terms of seconds. */
  { false }
  pub fn render(&mut self) -> bool
  { false }
}

