/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: client/state/console/console_renderer.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      Manages the client portion
      of the in-game console.
*/

use gl2 = opengles::gl2;
use gl;
use glfw;
use ui;
use math;
use super::State;
use console::Console;
use log::Log;

#[macro_escape]
#[path = "../../../shared/log/macros.rs"]
mod macros;

struct Console_Renderer
{
  console: @mut Console,

  position: math::Vec2f,
  velocity: f32, /* On the Y axis only. */

  font: ui::Font,

  tex_left: gl::Texture,
  tex_middle: gl::Texture,
  tex_right: gl::Texture,
}

impl Console_Renderer
{
  pub fn new(model: @mut Console) -> @mut Console_Renderer
  {
    let c = @mut Console_Renderer
    {
      console: model,

      position: math::Vec2f::zero(),
      velocity: 300.0,

      font: ui::Font::new("data/fonts/test.ttf", 16),

      tex_left: gl::Texture::new(gl2::TEXTURE_2D, "data/img/console/left.png"),
      tex_right: gl::Texture::new(gl2::TEXTURE_2D, "data/img/console/right.png"),
      tex_middle: gl::Texture::new(gl2::TEXTURE_2D, "data/img/console/middle.png"),
    };

    c
  }
}

impl State for Console_Renderer
{
  fn load(&mut self)
  { log_debug!("Loading console renderer state"); }

  fn unload(&mut self)
  { log_debug!("Unloading console renderer state"); }

  fn get_key(&self) -> &str
  { &"console_renderer" }

  fn update(&mut self, delta: f32) -> bool /* dt is in terms of seconds. */
  {
    self.position.y += (self.velocity * delta);
    self.position.y = self.position.y.clamp(&(-(self.tex_left.size.y + 1) as f32), &0.0);

    false
  }

  fn render(&mut self) -> bool
  {
    let renderer = ui::Renderer::get();
    renderer.begin();

    if self.position.y < -self.tex_left.size.y as f32
    {
      renderer.end();
      return false;
    }

    let right_pos =
      match renderer.window.get_size()
      { (width, _height) => math::Vec2f::new((width as i32 - self.tex_right.size.x) as f32, self.position.y) };

    let middle_pos = math::Vec2f::new(self.tex_left.size.x as f32, self.position.y);
    let middle_size = math::Vec2f::new(right_pos.x - self.tex_left.size.x as f32, self.tex_middle.size.y as f32);

    renderer.render_texture(&self.tex_left, &self.position);
    renderer.render_texture(&self.tex_right, &right_pos);
    renderer.render_texture_scale_clamp(&self.tex_middle, &middle_pos, &middle_size);

    renderer.render_font(self.console.body, math::Vec2f::new(self.tex_left.size.x as f32, 0.0 + self.position.y), &self.font);
    renderer.render_font
    (
      self.console.prefix,
      math::Vec2f::new
      (
        self.tex_left.size.x as f32,
        self.tex_left.size.y as f32 - 35.0 + self.position.y
      ), 
      &self.font
    );
    renderer.render_font
    (
      self.console.input,
      math::Vec2f::new
      (
        self.tex_left.size.x as f32 + 20.0,
        self.tex_left.size.y as f32 - 35.0 + self.position.y
      ),
      &self.font
    );

    renderer.end();

    false
  }

  fn key_action(&mut self, key: glfw::Key, action: glfw::Action, _mods: glfw::Modifiers) -> bool
  {
    if action == glfw::Press || action == glfw::Repeat
    {
      /* Mac grave is world 1 for some reason. */
      if key == glfw::KeyGraveAccent || key == glfw::KeyWorld1 
      {
        self.velocity *= -1.0;
        return true;
      }

      /* The following only apply if the console is enabled. */
      if self.velocity > 0.0
      {
        if key == glfw::KeyEnter
        {
          if self.console.input.len() == 0
          { return true; }

          /* Run the function and add the output to the log. */
          let input = self.console.input.clone();
          let (_res, output) = Console::run_function(input);
          self.console.add_log(output);

          self.console.input.clear();
        }
        else if key == glfw::KeyBackspace
        {
          if self.console.input.len() > 0
          { self.console.input.pop_char(); }
        }
        /* Non-whitespace. */
        else if key as i32 >= 32 && key as i32 <= 93
        {
          /* This will be handled when we receive it as a char. */
        }

        return true;
      }
    }

    false
  }

  fn key_char(&mut self, ch: char) -> bool
  {
    /* Check if the console is enabled. */
    if self.velocity > 0.0
    {
      /* Non-whitespace and not ` or ~ */
      if ch >= 0x20u8 as char && ch <= 0x7Du8 as char && ch != 0x60u8 as char
      {
        self.console.input.push_char(ch);
        return true;
      }
    }

    false
  }
}

