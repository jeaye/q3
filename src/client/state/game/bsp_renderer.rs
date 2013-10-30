/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: client/state/game/bsp_renderer.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      A client-only state that depends on
      the shared game state. This state is
      used only in development to testing
      the loading and rendering of Quake's
      BSP maps.
*/

use super::{ State, Game_Renderer };
use std::mem;
use gl2 = opengles::gl2;
use gl;
use glfw;
use ui;
use math;
use obj::bsp;
use log::Log;

#[macro_escape]
#[path = "../../../shared/log/macros.rs"]
mod macros;

#[macro_escape]
#[path = "../../gl/check.rs"]
mod check;

pub struct BSP_Renderer
{
  game_renderer: @mut Game_Renderer,

  vao: gl2::GLuint,
  vbo: gl2::GLuint, 

  shader: @mut gl::Shader,
  proj_loc: gl2::GLint,
  world_loc: gl2::GLint,
}

impl BSP_Renderer
{
  pub fn new(game_renderer: @mut Game_Renderer) -> @mut BSP_Renderer
  {
    let gr = @mut BSP_Renderer
    {
      game_renderer: game_renderer,

      vao: 0,
      vbo: 0,

      shader: gl::Shader_Builder::new_with_files("data/shaders/color.vert", "data/shaders/color.frag"),
      proj_loc: 0,
      world_loc: 0,
    };

    gr.upload();

    gr
  }

  fn upload(&mut self)
  {
    let name = check!(gl2::gen_vertex_arrays(1));
    log_assert!(name.len() == 1);
    self.vao = name[0];

    let name = check!(gl2::gen_buffers(1));
    log_assert!(name.len() == 1);
    self.vbo = name[0];

    check!(gl2::bind_vertex_array(self.vao));
    check!(gl2::bind_buffer(gl2::ARRAY_BUFFER, self.vbo));
    check!(gl2::buffer_data(gl2::ARRAY_BUFFER, self.game_renderer.game.bsp_map.verts, gl2::STATIC_DRAW));

    /* Setup vertex attribs. */
    check!(gl2::bind_buffer(gl2::ARRAY_BUFFER, self.vbo));
    check!(gl2::enable_vertex_attrib_array(0));
    check!(gl2::enable_vertex_attrib_array(1));

    check!(gl2::vertex_attrib_pointer_f32(0, 3, false, 
                mem::size_of::<bsp::lump::Vertex>() as i32, 
                0));
    check!(gl2::vertex_attrib_pointer_u8(1, 4, true, 
                mem::size_of::<bsp::lump::Vertex>() as i32, 
                mem::size_of::<bsp::lump::Vertex>() as u32 -
                mem::size_of::<math::Vec4u8>() as u32));
  }

  fn render_mesh(&self)
  {
    check!(gl2::bind_vertex_array(self.vao));
    check!(gl2::draw_arrays(gl2::TRIANGLES, 0, self.game_renderer.game.bsp_map.verts.len() as i32));
    check!(gl2::bind_vertex_array(0));
  }
}

impl State for BSP_Renderer
{
  fn load(&mut self)
  {
    log_debug!("Loading bsp renderer state.");

    self.game_renderer.camera.show_fps = true;

    self.shader.bind();
    self.proj_loc = self.shader.get_uniform_location("proj");
    self.world_loc = self.shader.get_uniform_location("world");
  }

  fn unload(&mut self)
  {
    log_debug!("Unloading bsp renderer state.");

    /* Cleanup GL. */
    check!(gl2::delete_vertex_arrays(&[self.vao]));
    check!(gl2::delete_buffers(&[self.vbo]));
  }

  fn get_key(&self) -> &str
  { &"bsp_renderer" }

  fn update(&mut self, delta: f32) -> bool /* dt is in terms of seconds. */
  {
    self.game_renderer.camera.update(delta);

    false
  }

  fn render(&mut self) -> bool
  {
    self.shader.bind();
    self.shader.update_uniform_mat(self.proj_loc, &self.game_renderer.camera.projection);
    self.shader.update_uniform_mat(self.world_loc, &self.game_renderer.camera.view);

    self.render_mesh();

    let fps = self.game_renderer.camera.frame_rate;

    let ui_renderer = ui::Renderer::get();
    ui_renderer.begin();
    {
      if self.game_renderer.camera.show_fps
      {
        ui_renderer.render_font(
          format!("{}", fps), 
          math::Vec2f::new(self.game_renderer.camera.window_size.x as f32 - 40.0, 0.0), 
          &self.game_renderer.fps_font); 
      }
    } ui_renderer.end();

    false
  }

  fn key_action(&mut self, key: glfw::Key, action: glfw::Action, _mods: glfw::Modifiers) -> bool
  { (self.game_renderer.camera as @mut State).key_action(key, action, _mods) }
  fn mouse_moved(&mut self, x: f32, y: f32) -> bool
  { (self.game_renderer.camera as @mut State).mouse_moved(x, y) }
}

