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

use std::{ libc, rt };
use gl2 = opengles::gl2;
use util::Log;

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

#[path = "util/mod.rs"]
pub mod util;

#[macro_escape]
#[path = "util/log_macros.rs"]
mod log_macros;

#[start]
fn main(argc: int, argv: **u8, crate_map: *u8) -> int
{
  do rt::start_on_main_thread(argc, argv, crate_map)
  {
    util::Log::initialize(); /* Main thread. */
    glfw::set_error_callback(error_callback);

    do glfw::start
    {
      glfw::window_hint::context_version(3, 2);
      glfw::window_hint::opengl_profile(glfw::OPENGL_CORE_PROFILE);
      glfw::window_hint::opengl_forward_compat(true);

      glfw::window_hint::visible(false);
      let window_res = glfw::Window::create(1, 1, "", glfw::Windowed);
      let worker_window = match window_res
      {
        Ok(win) => { win },
        Err(()) => { log_fail!("Failed to create worker window") }
      };

      glfw::window_hint::visible(true);
      let window_res = worker_window.create_shared(1024, 768, "Q³", glfw::Windowed);
      let window = match window_res
      {
        Ok(win) => { @win },
        Err(()) => { log_fail!("Failed to create main window") }
      };
      window.make_context_current();

      /* Start the background GL thread. */
      let gl_worker_port = gl::Worker::initialize(worker_window);
      let _ui_renderer = ui::Renderer::new(window);

      /* Create the console state. */
      state::Director::create();
      let console_state = state::Console::new();
      let console_renderer_state = state::Console_Renderer::new(console_state);
      do state::Director::get_mut |director|
      {
        director.push(console_state as @mut state::State);
        director.push(console_renderer_state as @mut state::State);
      }

      /* Initialize the default camera. */
      let cam = gl::Camera::new(window);
      cam.init();
      gl::Camera::set_active(cam);
      do window.set_size_callback |_, width, height|
      { gl::Camera::get_active().resize(width as i32, height as i32); }

      /* Setup callbacks. */
      do window.set_focus_callback |window, focused|
      { if focused { window.set_cursor_mode(glfw::CURSOR_DISABLED); } }
      do window.set_cursor_pos_callback |_, x, y| 
      { state::Director::mouse_moved(x as f32, y as f32); }
      do window.set_char_callback |_, c|
      { state::Director::key_char(c); }
      do window.set_key_callback |window, key, _scancode, action, mods|
      {
        state::Director::key_action(key, action, mods);
        key_callback(window, key, action);
      }

      let mut _model = md5::Model::new(~"data/models/berserker/berserker.md5mesh");
      log_assert!(_model.load_animation(~"data/models/berserker/idle.md5anim"));
      let mut _model_renderer = md5::Model_Renderer::new(&_model);

      /* Console functions. */
      state::Console::get().add_accessor("q3.version", |_|
      { fmt!("%s.%s", env!("VERSION"), env!("COMMIT")) });
      state::Console::get().add_function(~"quit", |_, _| -> (bool, ~str)
      { window.set_should_close(true); (true, ~"")});
      state::Console::get().add_function(~"load_map", |_, map_name| -> (bool, ~str)
      {
        let mut err = ~"";

        /* Try to load the new map. */
        let game_state = state::Game::new(map_name);
        if game_state.is_err()
        { err = game_state.unwrap_err(); }
        else
        {
          let game_state = game_state.unwrap();

          let game_renderer_state = state::Game_Renderer::new(game_state);
          do state::Director::get_mut |director|
          {
            /* Remove any existing game states. */
            do director.remove_if |state|
            { state.get_key() == (game_state as @mut state::State).get_key() || 
              state.get_key() == (game_renderer_state as @mut state::State).get_key() }

            director.unshift(game_state as @mut state::State);
            director.unshift(game_renderer_state as @mut state::State);
          }
          gl::Camera::get_active().reset(); /* Jump back to the origin. */
        }

        if err.len() > 0
        { (false, ~"\\2Error: \\1" + err) }
        else
        { (true, ~"Loaded map: \\5" + map_name + "\\1") }
      });
      /* Load the default map. */
      let (_loaded, msg) = state::Console::run_function(~"load_map q3ctf1");
      state::Console::get().add_log(msg);

      /* Delta time. */
      let mut cur_time = extra::time::precise_time_s() as f32;
      let mut last_time = cur_time;

      /* Enter game loop. */
      while !window.should_close()
      {
        glfw::poll_events();

        let delta = cur_time - last_time;
        last_time = cur_time;
        cur_time = extra::time::precise_time_s() as f32;

        state::Director::update(delta);

        _model_renderer.update(delta);

        check!(gl2::clear(gl2::COLOR_BUFFER_BIT | gl2::DEPTH_BUFFER_BIT));
        {
          _model_renderer.render();
          state::Director::render();
        } window.swap_buffers();
      }

      /* Kill the worker. */
      do gl::Worker::new_task
      { true } 
      gl_worker_port.recv(); /* Wait for the worker to finish. */
    }
  }
}

fn key_callback(window: &glfw::Window, key: libc::c_int, action: libc::c_int)
{
  if action == glfw::PRESS && key == glfw::KEY_ESCAPE
  { window.set_should_close(true); }
}

fn error_callback(error: libc::c_int, description: ~str)
{ log_error!("GLFW %d: %s", error as int, description); }

