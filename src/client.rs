/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: client.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      Main engine controller for the
      client-side game. Owns the window
      and runs the game loop.
*/

use std::comm;
use extra;

use glfw;
use gl2 = opengles::gl2;
use gl;
use console;
use ui;
use obj;
use state;
use log::Log;

#[macro_escape]
#[path = "gl/check.rs"]
mod check;

#[macro_escape]
#[path = "log/macros.rs"]
mod macros;

pub struct Client
{
  main_window: @glfw::Window,
  gl_worker_port: comm::Port<bool>,

  cur_time: f32,
  last_time: f32,
}

impl Client
{
  pub fn new() -> @mut Client
  {
    /* Let GLFW know how to report errors. */
    do glfw::set_error_callback |error, description|
    { log_fail!("GLFW %d: %s", error as int, description); }
    
    if glfw::init().is_err()
    { log_fail!("Failed to initialize GLFW"); }
    else
    {
      /* Create our OpenGL context. */
      log_debug!("Setting up GL context");
      glfw::window_hint::context_version(3, 2);
      glfw::window_hint::opengl_profile(glfw::OpenGlCoreProfile);
      glfw::window_hint::opengl_forward_compat(true);

      /* Create a worker window. */
      log_debug!("Creating worker window");
      glfw::window_hint::visible(false);
      let window_res = glfw::Window::create(1, 1, "", glfw::Windowed);
      let worker_window = match window_res
      {
        Ok(win) => { win },
        Err(()) => { log_fail!("Failed to create worker window") }
      };

      /* Split the worker window's context into the main window. */
      log_debug!("Creating main window");
      glfw::window_hint::visible(true);
      let window_res = worker_window.create_shared(1024, 768, "Q³", glfw::Windowed);
      let window = match window_res
      {
        Ok(win) => { @win },
        Err(()) => { log_fail!("Failed to create main window") }
      };

      let client = @mut Client
      {
        main_window: window,
        gl_worker_port: gl::Worker::initialize(worker_window),

        cur_time: 0.0,
        last_time: 0.0
      };

      client
    }
  }

  pub fn run(@mut self)
  {
    log_debug!("Binding GL context");
    self.main_window.make_context_current();

    let _ui_renderer = ui::Renderer::new(self.main_window);

    /* Create the console state. */
    state::Director::create();
    let console_state = console::Console::new();
    let console_renderer_state = state::Console_Renderer::new(console_state);
    do state::Director::get_mut |director|
    { director.push(console_renderer_state as @mut state::State); }

    /* Initialize the default camera. */
    let cam = gl::Camera::new(self.main_window);
    (cam as @mut state::State).load();
    gl::Camera::set_active(cam);
    do self.main_window.set_size_callback |_, width, height|
    { gl::Camera::get_active().resize(width as i32, height as i32); }

    /* Setup callbacks. */
    do self.main_window.set_focus_callback |window, focused|
    { if focused { window.set_cursor_mode(glfw::CursorDisabled); } }
    do self.main_window.set_cursor_pos_callback |_, x, y| 
    { state::Director::mouse_moved(x as f32, y as f32); }
    do self.main_window.set_char_callback |_, c|
    { state::Director::key_char(c); }
    do self.main_window.set_key_callback |window, key, _scancode, action, mods|
    { key_callback(window, key, action, mods); }

    let mut _model = obj::md5::Model::new(~"data/models/berserker/berserker.md5mesh");
    log_assert!(_model.load_animation(~"data/models/berserker/idle.md5anim"));
    let mut _model_renderer = obj::md5::Model_Renderer::new(&_model);

    /* Console functions. */
    console::Console::get().add_function(~"quit", self as @mut console::Functor);
    console::Console::get().add_function(~"load_map", self as @mut console::Functor);

    /* Load the default map. */
    let (_loaded, msg) = console::Console::run_function(~"load_map q3ctf1");
    console::Console::get().add_log(msg);

    /* Delta time. */
    self.cur_time = extra::time::precise_time_s() as f32;
    self.last_time = self.cur_time;

    /* Enter game loop. */
    log_debug!("Entering game loop");
    while !self.main_window.should_close()
    {
      glfw::poll_events();

      let delta = self.cur_time - self.last_time;
      self.last_time = self.cur_time;
      self.cur_time = extra::time::precise_time_s() as f32;

      state::Director::update(delta);
      _model_renderer.update(delta);

      check!(gl2::clear(gl2::COLOR_BUFFER_BIT | gl2::DEPTH_BUFFER_BIT));
      {
        state::Director::render();
        _model_renderer.render();
      } self.main_window.swap_buffers();
    }

    /* Kill the worker. */
    do gl::Worker::new_task
    { true } 
    self.gl_worker_port.recv(); /* Wait for the worker to finish. */

    /* Cleanup TLS. */
    state::Director::destroy();
  }
}

fn key_callback(window: &glfw::Window, key: glfw::Key, action: glfw::Action, mods: glfw::Modifiers)
{
  state::Director::key_action(key, action, mods);

  if action == glfw::Press && key == glfw::KeyEscape
  { window.set_should_close(true); }
}

impl console::Functor for Client
{
  fn call(&mut self, name: &str, params: &str) -> (bool, ~str)
  {
    match name
    {
      "quit" =>
      {
        self.main_window.set_should_close(true);
        (true, ~"Quitting")
      },
      "load_map" =>
      {
        let mut err = ~"";

        /* Try to load the new map. */
        let game_state = state::Game::new(params);
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
        { (true, ~"Loaded map: \\5" + params + "\\1") }
      }

      _ => (false, ~"ERROR"),
    }
  }
}

