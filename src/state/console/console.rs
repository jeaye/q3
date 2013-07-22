/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: state/console/console.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      Manages the model portion
      of the in-game console.
*/

use glfw;
use ui;
use math;
use super::State;

struct Console
{
  activator: @mut ui::Console_Activator,

  position: math::Vec2f,
  velocity: f32, /* On the Y axis only. */
  
  body: ~str,
  prefix: ~str,
  input: ~str,
}

impl Console
{
  pub fn new() -> @mut Console
  {
    let c = @mut Console
    {
      activator: ui::Console_Activator::new(),

      position: math::Vec2f::zero(),
      velocity: 300.0,

      body: fmt!("Welcome to Q^3\nVersion: %s.%s", env!("VERSION"), env!("COMMIT")),
      prefix: ~"> ",
      input: ~"", 
    };

    /* The 'get' and 'set' functions are built in to the console. */
    c.activator.functions.insert(~"get",
    |_get, property| -> Option<~str>
    {
      let mut err = ~"";

      /* Check if this property exists. */
      match c.activator.accessors.find(&property.to_owned())
      {
        Some(func) =>
        { c.add_log(fmt!("%s = %s", property, (*func)(property))); }
        None =>
        { err = fmt!("Error: Invalid property %s", property); }
      }

      if err.len() == 0
      { None }
      else
      { Some(err) }
    });

    c.activator.functions.insert(~"set",
    |_set, params| -> Option<~str>
    {
      let mut error = ~"";
      let mut property = ~"";
      let mut params = params.to_owned();
      for params.split_iter(' ').advance |x|
      { property = x.to_owned(); break; }

      /* Remove the property from the string. */
      for property.iter().advance |_|
      { params.shift_char(); }
      params.shift_char();

      /* We require a property and a value. */
      if property.len() == 0
      { error = fmt!("Invalid len for 'set' function arg list: %s", params); }
      else
      {
        /* Check if this property exists. */
        match c.activator.mutators.find(&property)
        {
          Some(func) =>
          {
            /* Pass the args to the property mutator. */
            match (*func)(property, params)
            {
              /* Check if the mutator liked the args. */
              Some(err) => { error = err; }
              None => { }
            }
          }
          None => { error = fmt!("The property %s does not exist.", property); }
        }
      }

      if error.len() > 0
      { Some(fmt!("Error: %s", error)) }
      else
      { None }
    });


    c
  }

  pub fn add_log(&mut self, text: &str)
  { self.body.push_str("\n" + text); }
}

impl State for Console
{
  pub fn load(&mut self)
  { debug!("Loading console state."); }

  pub fn unload(&mut self)
  { debug!("Unloading console state."); }

  pub fn update(&mut self, _delta: f32) -> bool /* dt is in terms of seconds. */
  { false }

  pub fn render(&mut self) -> bool
  { false }

  pub fn key_action(&mut self, key: i32, action: i32, _mods: i32) -> bool
  {
    if action == glfw::PRESS || action == glfw::REPEAT
    {
      /* Mac grave is world 1 for some reason. */
      if key == glfw::KEY_GRAVE_ACCENT || key == glfw::KEY_WORLD_1 
      {
        self.velocity *= -1.0;
        return true;
      }

      /* The following only apply if the console is enabled. */
      if self.velocity > 0.0
      {
        if key == glfw::KEY_ENTER
        {
          if self.input.len() == 0
          { return true; }

          /* Extract the function name. */
          let mut func = ~"";
          for self.input.split_iter(' ').advance |x|
          { func = x.to_owned(); break; };

          /* Remove the function from the string. */
          for func.iter().advance |_|
          { self.input.shift_char(); }
          self.input.shift_char();

          /* Look for the function in the cached map. */
          match self.activator.functions.find(&func)
          {
            Some(f) =>
            {
              let input = self.input.clone();
              match (*f)(func, input)
              {
                Some(err) => { self.add_log(err); }
                None => { }
              }
            }
            None => { self.add_log("Error: Invalid function"); }
          }

          self.input = ~"";
          return true;
        }
        else if key == glfw::KEY_BACKSPACE
        {
          if self.input.len() > 0
          { self.input.pop_char(); }
          return true;
        }
        /* Non-whitespace. */
        else if key >= 32 && key <= 93
        {
          /* This will be handled when we receive it as a char. */
          return true;
        }
      }
    }

    false

  }

  pub fn key_char(&mut self, ch: char) -> bool
  {
    /* Check if the console is enabled. */
    if self.velocity > 0.0
    {
      /* Non-whitespace and not ` or ~ */
      if ch >= 0x20 as char && ch <= 0x7D as char && ch != 0x60 as char
      {
        self.input.push_char(ch);
        return true;
      }
    }

    false
  }
}

