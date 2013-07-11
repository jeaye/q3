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

priv type Property_Accessor = @fn(&str) -> ~str;
priv type Property_Mutator = @fn(&str, &str) -> Option<~str>;
priv type Function = @fn(&str, &str) -> Option<~str>;

pub struct Console_Activator
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

impl Console_Activator
{
  /*  Key function used to index our singleton in
      task-local storage. */
  priv fn tls_key(_: @@Console_Activator) { }

  pub fn new() -> @mut Console_Activator
  {
    let ca = @mut Console_Activator
    {
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
}

