/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: console/console.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      Manages the model portion
      of the in-game console.
*/

use std::local_data;
use std::hashmap::HashMap;
use log::Log;

#[macro_escape]
#[path = "../log/macros.rs"]
mod macros;

/* Takes: property name; Returns: property value. */
type Property_Accessor = @fn(&str) -> ~str; 
/* Takes: property name, new value; Returns: Some(failure message). */
type Property_Mutator = @fn(&str, &str) -> Option<~str>;
/* Takes: function name, params; Returns: sucess/failure, message. */
type Function = @fn(&str, &str) -> (bool, ~str);

static tls_key: local_data::Key<@mut Console> = &local_data::Key;

struct Registry
{
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

struct Console
{
  body: ~str,
  prefix: ~str,
  input: ~str,

  registry: Registry,
}

impl Console
{
  pub fn new() -> @mut Console
  {
    let c = @mut Console
    {
      body: fmt!("Welcome to Q³\nVersion: %s.%s", env!("VERSION"), env!("COMMIT")),
      prefix: ~"> ",
      input: ~"", 

      registry: Registry
      {
        accessors: HashMap::<~str, Property_Accessor>::new(),
        mutators: HashMap::<~str, Property_Mutator>::new(),
        functions: HashMap::<~str, Function>::new(),
      },
    };

    /* Store the console in task-local storage. (singleton) */
    local_data::set(tls_key, c);

    /* The 'get' and 'set' functions are built in to the console. */
    c.registry.functions.insert(~"get",
    |_get, property| -> (bool, ~str)
    {
      let mut msg;
      let mut success = false;

      /* Check if this property exists. */
      match c.registry.accessors.find(&property.to_owned())
      {
        Some(func) =>
        { msg = fmt!("%s = %s", property, (*func)(property)); success = true; }
        None =>
        { msg = fmt!("\\\\2Error: \\\\1Invalid property '%s'", property); }
      }

      (success, msg)
    });

    c.registry.functions.insert(~"set",
    |_set, params| -> (bool, ~str)
    {
      let mut msg;
      let mut success = false;
      let mut property = ~"";
      let mut params = params.to_owned();
      for x in params.split_iter(' ')
      { property = x.to_owned(); break; }

      /* We require a property and a value.
       * Remove the property from the string.
       */
      for _ in property.iter()
      { params.shift_char(); }
      if params.len() > 0
      { params.shift_char(); }

      /* Check if this property exists. */
      match c.registry.mutators.find(&property)
      {
        Some(func) =>
        {
          /* Pass the args to the property mutator. */
          match (*func)(property, params)
          {
            /* Check if the mutator liked the args. */
            Some(err) => { success = false; msg = ~"\\\\2Error: \\\\1" + err; }
            None => { success = true; msg = fmt!("\\\\1%s = %s", property, params); }
          }
        }
        None => { msg = fmt!("\\\\2Error: \\\\1The property '%s' does not exist.", property); }
      }

      (success, msg)
    });


    c
  }

  /* Accesses the singleton consol from task-local storage. */
  pub fn get() -> @mut Console
  {
    local_data::get(tls_key, 
    |opt|
    {
      match opt
      {
        Some(x) => *x,
        None => log_fail!("Singleton not available")
      }
    })
  }

  pub fn add_function(&mut self, name: ~str, func: Function)
  { self.registry.functions.insert(name, func); }
  pub fn add_accessor(&mut self, name: &str, accessor: Property_Accessor)
  { self.registry.accessors.insert(name.to_owned(), accessor); }
  pub fn add_mutator(&mut self, name: &str, mutator: Property_Mutator)
  { self.registry.mutators.insert(name.to_owned(), mutator); }
  pub fn add_log(&mut self, text: &str)
  { self.body.push_str("\n" + text); }
  pub fn add_error_log(&mut self, text: &str)
  { self.body.push_str("\n\\2Error: \\1" + text); }

  pub fn run_function(mut input_func: ~str) -> (bool, ~str)
  {
    /* Extract the function name. */
    let mut func = ~"";
    input_func = input_func.trim().to_owned();
    for x in input_func.split_iter(' ')
    { func = x.to_owned(); break; };

    /* Remove the function from the string. */
    for _ in func.iter()
    { input_func.shift_char(); }
    if input_func.len() > 0
    { input_func.shift_char(); }

    /* Look for the function in the cached map. */
    let fun = Console::get().registry.functions.find(&func);
    match fun
    {
      Some(f) =>
      {
        let input = input_func.clone();
        (*f)(func, input)
      }
      None => { (false, fmt!("\\\\2Error: \\\\1Invalid function '%s'", func)) }
    }
  }
}

