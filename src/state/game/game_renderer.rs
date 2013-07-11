/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: state/game/game_renderer.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      A client-only state that depends on
      the shared game state. This state owns the
      camera and uses it to render the map data.
*/

use glfw;
use super::{ State, Game };
use gl2 = opengles::gl2;
use gl;
use ui;
use math;

#[path = "../../gl/check.rs"]
mod check;

pub struct Game_Renderer
{
  game: @mut Game,
  camera: @mut gl::Camera,

  shader: @gl::Shader,
  proj_loc: gl2::GLint,
  world_loc: gl2::GLint,
  voxel_size_loc: gl2::GLint,
  offsets_loc: gl2::GLint,

  fps_font: ui::Font,
}

impl Game_Renderer
{
  pub fn new(game: @mut Game, window: @glfw::Window) -> @mut Game_Renderer
  {
    let gr = @mut Game_Renderer
    {
      game: game,
      camera: gl::Camera::new(window),

      shader: gl::Shader_Builder::new_with_files("data/shaders/voxel.vert", "data/shaders/voxel.frag"),
      proj_loc: 0,
      world_loc: 0,
      voxel_size_loc: 0,
      offsets_loc: 0,

      fps_font: ui::Font::new("data/fonts/test.ttf", 30),
    };

    gr.camera.init();

    do window.set_size_callback |_, width, height|
    { gr.camera.resize(width as i32, height as i32); }

    gr
  }
}

impl State for Game_Renderer
{
  pub fn load(&mut self)
  {
    debug!("Loading game renderer state.");

    self.shader.bind();
    self.proj_loc = self.shader.get_uniform_location("proj");
    self.world_loc = self.shader.get_uniform_location("world");
    self.voxel_size_loc = self.shader.get_uniform_location("voxel_size");
    self.offsets_loc = self.shader.get_uniform_location("offsets");

    self.shader.update_uniform_i32(self.offsets_loc, 0);
  }

  pub fn unload(&mut self)
  { debug!("Unloading game renderer state."); }

  pub fn update(&mut self, delta: f32) -> bool /* dt is in terms of seconds. */
  {
    self.camera.update(delta);

    false
  }

  pub fn render(&mut self) -> bool
  {
    self.shader.bind();
    self.shader.update_uniform_mat(self.proj_loc, &self.camera.projection);
    self.shader.update_uniform_mat(self.world_loc, &self.camera.view);
    self.shader.update_uniform_f32(self.voxel_size_loc, self.game.voxel_map.voxel_size);

    self.game.voxel_map.draw();

    let fps = self.camera.frame_rate;

    let ui_renderer = ui::Renderer::get();
    ui_renderer.begin();
    if self.camera.show_fps
    { ui_renderer.render_font(fmt!("%?", fps), math::Vec2f::new(self.camera.window_size.x as f32 - 40.0, 0.0), &self.fps_font); }
    ui_renderer.end();

    false
  }

  pub fn key_action(&mut self, key: i32, action: i32, _mods: i32) -> bool
  { (self.camera as @mut ui::Input_Listener).key_action(key, action, _mods) }
  pub fn mouse_moved(&mut self, x: f32, y: f32) -> bool
  { (self.camera as @mut ui::Input_Listener).mouse_moved(x, y) }
}

