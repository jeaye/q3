/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: state/console/console_renderer.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      Manages the view portion
      of the in-game console.
*/

use gl2 = opengles::gl2;
use gl;
use ui;
use math;
use super::{ State, Console };
use util::Log;

#[macro_escape]
#[path = "../../util/log_macros.rs"]
mod log_macros;

struct Console_Renderer
{
  console: @mut Console,

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
  pub fn load(&mut self)
  { log_debug!("Loading console renderer state"); }

  pub fn unload(&mut self)
  { log_debug!("Unloading console renderer state"); }

  pub fn update(&mut self, delta: f32) -> bool /* dt is in terms of seconds. */
  {
    self.console.position.y += (self.console.velocity * delta);
    self.console.position.y = self.console.position.y.clamp(&(-(self.tex_left.size.y + 1) as f32), &0.0);

    false
  }

  pub fn render(&mut self) -> bool
  {
    let renderer = ui::Renderer::get();
    renderer.begin();

    if self.console.position.y < -self.tex_left.size.y as f32
    {
      renderer.end();
      return false;
    }

    let right_pos =
      match renderer.window.get_size()
      { (width, _height) => math::Vec2f::new((width as i32 - self.tex_right.size.x) as f32, self.console.position.y) };

    let middle_pos = math::Vec2f::new(self.tex_left.size.x as f32, self.console.position.y);
    let middle_size = math::Vec2f::new(right_pos.x - self.tex_left.size.x as f32, self.tex_middle.size.y as f32);

    renderer.render_texture(&self.tex_left, &self.console.position);
    renderer.render_texture(&self.tex_right, &right_pos);
    renderer.render_texture_scale_clamp(&self.tex_middle, &middle_pos, &middle_size);

    renderer.render_font(self.console.body, math::Vec2f::new(self.tex_left.size.x as f32, 0.0 + self.console.position.y), &self.font);
    renderer.render_font
    (
      self.console.prefix,
      math::Vec2f::new
      (
        self.tex_left.size.x as f32,
        self.tex_left.size.y as f32 - 35.0 + self.console.position.y
      ), 
      &self.font
    );
    renderer.render_font
    (
      self.console.input,
      math::Vec2f::new
      (
        self.tex_left.size.x as f32 + 20.0,
        self.tex_left.size.y as f32 - 35.0 + self.console.position.y
      ),
      &self.font
    );

    renderer.end();

    false
  }
}

