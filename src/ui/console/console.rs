/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: ui/console/console.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      A drop-down UI console for in-game tweaking.
*/

use gl;
use gl2 = opengles::gl2;
use ui;
use math;

pub struct Console
{
  tex_left: gl::Texture,
  tex_middle: gl::Texture,
  tex_right: gl::Texture,

  position: math::Vec2f,
  velocity: f32, /* On the Y axis only. */
  
  font: ui::Font,
  body: ~str,
  prefix: ~str,
  input: ~str,
}

impl Console
{
  pub fn new() -> Console
  {
    let c = Console
    {
      tex_left: gl::Texture::new(gl2::TEXTURE_2D, "data/img/console/left.png"),
      tex_right: gl::Texture::new(gl2::TEXTURE_2D, "data/img/console/right.png"),
      tex_middle: gl::Texture::new(gl2::TEXTURE_2D, "data/img/console/middle.png"),

      position: math::Vec2f::zero(),
      velocity: 300.0,

      /* Text. */
      font: ui::Font::new("data/fonts/test.ttf", 16),
      body: ~"Welcome to Q^3\nVersion: 0.1",
      prefix: ~"> ",
      input: ~"", 
    };

    c
  }

  pub fn update(&mut self, dt: f32)
  {
    self.position.y += (self.velocity * dt);
    self.position.y = self.position.y.clamp(&(-(self.tex_left.size.y + 1) as f32), &0.0);
  }

  pub fn render(&self, renderer: @mut ui::Renderer)
  {
    if self.position.y < -self.tex_left.size.y as f32
    { return; }

    let right_pos =
      match renderer.window.get_size()
      { (width, _height) => math::Vec2f::new((width as i32 - self.tex_right.size.x) as f32, self.position.y) };

    let middle_pos = math::Vec2f::new(self.tex_left.size.x as f32, self.position.y);
    let middle_size = math::Vec2f::new(right_pos.x - self.tex_left.size.x as f32, self.tex_middle.size.y as f32);

    renderer.render_texture(&self.tex_left, &self.position);
    renderer.render_texture(&self.tex_right, &right_pos);
    renderer.render_texture_scale_clamp(&self.tex_middle, &middle_pos, &middle_size);

    renderer.render_font(self.body, math::Vec2f::new(self.tex_left.size.x as f32, 0.0 + self.position.y), &self.font);
    renderer.render_font
    (
      self.prefix,
      math::Vec2f::new
      (
        self.tex_left.size.x as f32,
        self.tex_left.size.y as f32 - 35.0 + self.position.y
      ), 
      &self.font
    );
    renderer.render_font
    (
      self.input,
      math::Vec2f::new
      (
        self.tex_left.size.x as f32 + 20.0,
        self.tex_left.size.y as f32 - 35.0 + self.position.y
      ),
      &self.font
    );
  }
}

