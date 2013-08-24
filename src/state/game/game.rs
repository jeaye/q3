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

use extra;
use BSP_Map = bsp::Map;
use Voxel_Map = voxel::Map;
use super::State;
use util::Log;

#[macro_escape]
#[path = "../../util/log_macros.rs"]
mod log_macros;

pub struct Game
{
  bsp_map: BSP_Map,
  voxel_map: @mut Voxel_Map,
}

impl Game
{
  pub fn new(map_name: &str) -> @mut Game
  {
    let bmap = BSP_Map::new(~"data/maps/" + map_name + ".bsp");

    let start_time = extra::time::precise_time_s();
    let vmap = Voxel_Map::new(bmap.tris, 300);
    let time = extra::time::precise_time_s() - start_time;
    log_info!("Voxelization took %? seconds", time);

    let game = @mut Game
    {
      voxel_map: vmap,
      bsp_map: bmap,
    };

    game
  }
}

impl State for Game
{
  fn load(&mut self)
  { log_debug!("Loading game state."); }
  fn unload(&mut self)
  { log_debug!("Unloading game state."); }

  fn get_key(&self) -> &str
  { &"game" }

  fn update(&mut self, _delta: f32) -> bool /* dt is in terms of seconds. */
  { false }
  fn render(&mut self) -> bool
  { false }
}

