/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: main.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      Entry point.
*/

extern mod extra;
extern mod opengles;
extern mod glfw;
extern mod stb_image;

use std::libc;
use gl2 = opengles::gl2;

/* TODO: Should be able to remove. */
#[cfg(target_os = "linux")]
#[nolink]
#[link_args = "-lX11 -lXrandr -lXi -lXxf86vm"]
extern { }

#[path = "gl/mod.rs"]
pub mod gl;

#[path = "math/mod.rs"]
pub mod math;

#[macro_escape]
#[path = "gl/check.rs"]
mod check;

#[path = "obj/bsp/mod.rs"]
pub mod bsp; 

#[path = "ui/mod.rs"]
pub mod ui;

#[path = "obj/primitive/mod.rs"]
pub mod primitive;

#[path = "obj/voxel/mod.rs"]
pub mod voxel;

#[path = "state/mod.rs"]
pub mod state;

fn main()
{
  glfw::set_error_callback(error_callback);

  do glfw::spawn
  {
    glfw::window_hint::context_version(3, 2);
    glfw::window_hint::opengl_profile(glfw::OPENGL_CORE_PROFILE);
    glfw::window_hint::opengl_forward_compat(true);

    let window_res = glfw::Window::create(1024, 768, "Q^3", glfw::Windowed);
    let window = match window_res
    {
      Some(win) => { @win },
      None => { fail!("Failed to create window!") }
    };
    window.make_context_current();

    let ui_renderer = ui::Renderer::new(window);

    let states = state::Director::new();
    let console_state = state::Console::new();
    let game_state = state::Game::new();
    let game_renderer_state = state::Game_Renderer::new(game_state, window);
    states.push(game_state as @mut state::State);
    states.push(game_renderer_state as @mut state::State);
    states.push(console_state as @mut state::State);

    /* Setup callbacks. */ /* TODO: Crash on close with these callbacks. */
    window.set_cursor_mode(glfw::CURSOR_DISABLED);
    do window.set_cursor_pos_callback |_, x, y| 
    { states.mouse_moved(x as f32, y as f32); }
    do window.set_char_callback |_, c|
    { states.key_char(c); }
    do window.set_key_callback |window, key, _scancode, action, mods|
    {
      states.key_action(key, action, mods);
      key_callback(window, key, action);
    }

    /* Shader Creation. */
    //let color_shader = @mut gl::Shader_Builder::new_with_files("data/shaders/color.vert", "data/shaders/color.frag");

    //let color_proj_loc = color_shader.get_uniform_location("proj");
    //let color_world_loc = color_shader.get_uniform_location("world");

    let mut cur_time = extra::time::precise_time_s() as f32;
    let mut last_time = cur_time;

    while !window.should_close()
    {
      glfw::poll_events();

      let delta = cur_time - last_time;
      last_time = cur_time;
      cur_time = extra::time::precise_time_s() as f32;

      //color_shader.bind();
      //color_shader.update_uniform_mat(color_proj_loc, &camera.projection);
      //color_shader.update_uniform_mat(color_world_loc, &camera.view);

      states.update(delta);

      check!(gl2::clear(gl2::COLOR_BUFFER_BIT | gl2::DEPTH_BUFFER_BIT));
      {
        //color_shader.bind();
        //map.draw();

        states.render();
      } window.swap_buffers();
    }
  }
}

fn key_callback(window: &glfw::Window, key: libc::c_int, action: libc::c_int)
{
  if action == glfw::PRESS && key == glfw::KEY_ESCAPE
  { window.set_should_close(true); }
}

fn error_callback(error: libc::c_int, description: ~str)
{ error!("GLFW Error %?: %s", error, description); }

