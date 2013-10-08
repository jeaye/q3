/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: client/state/game/game_renderer.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      A client-only state that depends on
      the shared game state. This state owns the
      camera and uses it to render the map data.
*/

use super::{ State, Game };
use glfw;
use gl;
use ui;
use math;
use super::Map_Renderer;
use log::Log;

#[macro_escape]
#[path = "../../gl/check.rs"]
mod check;

#[macro_escape]
#[path = "../../../shared/log/macros.rs"]
mod macros;

pub struct Game_Renderer
{
  game: @mut Game,
  camera: @mut gl::Camera,
  map_renderer: @mut Map_Renderer,

  fps_font: ui::Font,
}

impl Game_Renderer
{
  pub fn new(game: @mut Game) -> @mut Game_Renderer
  {
    let gr = @mut Game_Renderer
    {
      game: game,
      camera: gl::Camera::get_active(),
      map_renderer: Map_Renderer::new(game.voxel_map),

      fps_font: ui::Font::new("data/fonts/test.ttf", 30),
    };

    gr
  }

}

impl State for Game_Renderer
{
  fn load(&mut self)
  {
    log_debug!("Loading game renderer state");

    (self.map_renderer as @mut State).load();
  }

  fn unload(&mut self)
  {
    log_debug!("Unloading game renderer state");

    (self.map_renderer as @mut State).unload();
  }

  fn get_key(&self) -> &str
  { &"game_renderer" }

  fn update(&mut self, delta: f32) -> bool /* dt is in terms of seconds. */
  {
    self.camera.update(delta);
    self.map_renderer.update(delta);

    false
  }

  fn render(&mut self) -> bool
  {
    (self.map_renderer as @mut State).render();

    let fps = self.camera.frame_rate;

    let ui_renderer = ui::Renderer::get();
    ui_renderer.begin();
    if self.camera.show_fps
    { ui_renderer.render_font(fmt!("%d", fps as int), math::Vec2f::new(0.0, 0.0), &self.fps_font); }
    ui_renderer.render_font(fmt!("%s", self.camera.position.to_str()), math::Vec2f::new(0.0, self.camera.window_size.y as f32 - 40.0), &self.fps_font);
    ui_renderer.end();

    false
  }

  fn key_action(&mut self, key: glfw::Key, action: glfw::Action, _mods: glfw::Modifiers) -> bool
  { (self.camera as @mut State).key_action(key, action, _mods) }
  fn mouse_moved(&mut self, x: f32, y: f32) -> bool
  { (self.camera as @mut State).mouse_moved(x, y) }
}

