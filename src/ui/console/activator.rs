/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: ui/console/activator.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      An input listener to open/close the console.
*/

use std::{ cast, local_data };
use std::hashmap::HashMap;
use std::iterator::IteratorUtil;
use glfw::{ PRESS, REPEAT, KEY_GRAVE_ACCENT, KEY_ENTER, KEY_BACKSPACE };
use ui::Input_Listener;
use super::Console;

priv type Property_Accessor = @fn(&str) -> ~str;
priv type Property_Mutator = @fn(&str, &str) -> Option<~str>;
priv type Function = @fn(&str, &str) -> Option<~str>;

pub struct Console_Activator
{
  console: @mut Console,

  /*  
      Maps of property names to callbacks (get and set).
      Properties are invoked with a prebuilt 'set' or 'get'
      function. 
      
      The only argument to the accessor is the property name
      and it returns the value.

      The first argument of the mutator is the
      property name and the second is everything else 
      contained in the command. The mutator returns a string
      containing an error message if something didn't work well.

      Ex: set map.wireframe on
      Ex: get map.wireframe
  */
  accessors: HashMap<~str, Property_Accessor>,
  mutators: HashMap<~str, Property_Mutator>,

  /*
      A map of arbitrary functions to callbacks.

      The first argument is the function name and the second
      is whatever else is supplied in the command. The return
      value is a string containing an error message if something
      didn't go well.

      Ex: record my.avi
      Ex: callvote kick annoying_dude
  */
  functions: HashMap<~str, Function>,
}

impl Console_Activator
{
  /*  Key function used to index our singleton in
      task-local storage. */
  priv fn tls_key(_: @@Console_Activator) { }

  pub fn new(new_console: @mut Console) -> @mut Console_Activator
  {
    let ca = @mut Console_Activator
    {
      console: new_console,
      accessors: HashMap::new::<~str, Property_Accessor>(),
      mutators: HashMap::new::<~str, Property_Mutator>(),
      functions: HashMap::new::<~str, Function>(),
    };

    /* Store the activator in task-local storage. (singleton) */
    unsafe
    {
      local_data::local_data_set
      (
        Console_Activator::tls_key,
        @cast::transmute::<@mut Console_Activator, @Console_Activator>(ca)
      );
    }

    /* The 'get' and 'set' functions are built in to the console. */
    ca.functions.insert(~"get",
    |_get, property| -> Option<~str>
    {
      let mut err = ~"";

      /* Check if this property exists. */
      match ca.accessors.find(&property.to_owned())
      {
        Some(func) =>
        { ca.add_log(fmt!("%s = %s", property, (*func)(property))); }
        None =>
        { err = fmt!("Error: Invalid property %s", property); }
      }

      if err.len() == 0
      { None }
      else
      { Some(err) }
    });

    ca.functions.insert(~"set",
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
        match ca.mutators.find(&property)
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

    ca
  }

  /* Accesses the singleton activator from task-local storage. */
  pub fn get() -> @mut Console_Activator
  {
    unsafe 
    {
      cast::transmute::<@Console_Activator, @mut Console_Activator>
      (*local_data::local_data_get(Console_Activator::tls_key).get())
    }
  }

  pub fn add_accessor(&mut self, name: &str, accessor: Property_Accessor)
  { self.accessors.insert(name.to_owned(), accessor); }
  pub fn add_mutator(&mut self, name: &str, mutator: Property_Mutator)
  { self.mutators.insert(name.to_owned(), mutator); }
  pub fn add_log(&self, text: &str)
  { self.console.body = self.console.body + "\n" + text; }
}

impl Input_Listener for Console_Activator
{
  pub fn key_action(&mut self, key: i32, action: i32, _mods: i32) -> bool
  {
    if action == PRESS || action == REPEAT
    {
      if key == KEY_GRAVE_ACCENT
      {
        self.console.velocity *= -1.0;
        return true;
      }

      /* The following only apply if the console is enabled. */
      if self.console.velocity > 0.0
      {
        if key == KEY_ENTER
        {
          if self.console.input.len() == 0
          { return true; }

          /* Extract the function name. */
          let mut func = ~"";
          for self.console.input.split_iter(' ').advance |x|
          { func = x.to_owned(); break; };

          /* Remove the function from the string. */
          for func.iter().advance |_|
          { self.console.input.shift_char(); }
          self.console.input.shift_char();

          /* Look for the function in the cached map. */
          match self.functions.find(&func)
          {
            Some(f) =>
            {
              let input = copy self.console.input;
              match (*f)(func, input)
              {
                Some(err) => { self.add_log(err); }
                None => { }
              }
            }
            None => { self.add_log("Error: Invalid function"); }
          }

          self.console.input = ~"";
          return true;
        }
        else if key == KEY_BACKSPACE
        {
          if self.console.input.len() > 0
          { self.console.input.pop_char(); }
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
    if self.console.velocity > 0.0
    {
      /* Non-whitespace and not ` or ~ */
      if ch >= 0x20 as char && ch <= 0x7D as char && ch != 0x60 as char
      {
        self.console.input.push_char(ch);
        return true;
      }
    }

    false
  }
  pub fn mouse_action(&mut self, _button: i32, _action: i32, _mods: i32) -> bool
  { false }
  pub fn mouse_moved(&mut self, _x: f32, _y: f32) -> bool
  { false }
}

