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

#[path = "obj/md5/mod.rs"]
pub mod md5;

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
      Ok(win) => { @win },
      Err(()) => { fail!("Failed to create window!") }
    };
    window.make_context_current();

    let _ui_renderer = ui::Renderer::new(window);

    /* Create all the states we need. */
    let states = state::Director::new();
    let console_state = state::Console::new();
    let console_renderer_state = state::Console_Renderer::new(console_state);
    let game_state = state::Game::new();
    let game_renderer_state = state::Game_Renderer::new(game_state, window);
    let bsp_renderer_state = state::BSP_Renderer::new(game_renderer_state);
    states.push(game_state as @mut state::State);
    states.push(game_renderer_state as @mut state::State);
    states.push(console_state as @mut state::State);
    states.push(console_renderer_state as @mut state::State);

    /* Setup callbacks. */ /* TODO: Crash on close with these callbacks. */
    do window.set_focus_callback |_, focused|
    { if focused {window.set_cursor_mode(glfw::CURSOR_DISABLED); } }
    do window.set_cursor_pos_callback |_, x, y| 
    { states.mouse_moved(x as f32, y as f32); }
    do window.set_char_callback |_, c|
    { states.key_char(c); }
    do window.set_key_callback |window, key, _scancode, action, mods|
    {
      /* TODO: Ability to pop specific states or insert others. */
      if key == glfw::KEY_LEFT_BRACKET && action == glfw::PRESS
      {
        states.pop(); /* console renderer */
        states.pop(); /* console */
        states.pop(); /* renderer */
        states.push(game_renderer_state as @mut state::State);
        states.push(console_state as @mut state::State);
        states.push(console_renderer_state as @mut state::State);
      }
      else if key == glfw::KEY_RIGHT_BRACKET && action == glfw::PRESS
      {
        states.pop(); /* console renderer */
        states.pop(); /* console */
        states.pop(); /* renderer */
        states.push(bsp_renderer_state as @mut state::State);
        states.push(console_state as @mut state::State);
        states.push(console_renderer_state as @mut state::State);
      }

      states.key_action(key, action, mods);
      key_callback(window, key, action);
    }

    let _model = md5::Model::new(~"data/models/bob/bob.md5mesh");
    let _model_renderer = md5::Model_Renderer::new(&_model);

    /* Console functions. */
    ui::Console_Activator::get().add_accessor("q3.version", |_|
    { fmt!("%s.%s", env!("VERSION"), env!("COMMIT")) });

    /* Delta time. */
    let mut cur_time = extra::time::precise_time_s() as f32;
    let mut last_time = cur_time;

    while !window.should_close()
    {
      glfw::poll_events();

      let delta = cur_time - last_time;
      last_time = cur_time;
      cur_time = extra::time::precise_time_s() as f32;

      states.update(delta);

      check!(gl2::clear(gl2::COLOR_BUFFER_BIT | gl2::DEPTH_BUFFER_BIT));
      {
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

