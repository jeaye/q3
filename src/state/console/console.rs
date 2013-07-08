/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: state/console/console.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      Manages the model and view portion
      of the in-game console.
*/

use UI_Console = ui::Console;
use ui;
use super::{ State };

struct Console
{
  console: @mut UI_Console,
  activator: @mut ui::Console_Activator,
}

impl Console
{
  pub fn new() -> @mut Console
  {
    let view = @mut UI_Console::new();
    let c = @mut Console
    {
      console: view,
      activator: ui::Console_Activator::new(view),
    };

    c
  }
}

impl State for Console
{
  pub fn load(&mut self)
  {
    debug!("Loading console state."); 
  }

  pub fn unload(&mut self)
  { debug!("Unloading console state."); }

  pub fn update(&mut self, delta: f32) -> bool /* dt is in terms of seconds. */
  {
    self.console.update(delta);

    false
  }

  pub fn render(&mut self) -> bool
  {
    let renderer = ui::Renderer::get();
    renderer.begin();
    self.console.render(renderer);
    renderer.end();

    false
  }

  pub fn key_action(&mut self, key: i32, action: i32, _mods: i32) -> bool
  { (self.activator as @mut ui::Input_Listener).key_action(key, action, _mods) }
  pub fn key_char(&mut self, ch: char) -> bool
  { (self.activator as @mut ui::Input_Listener).key_char(ch) }
}

