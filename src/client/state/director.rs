/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: client/state/director.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      A state stack that propogates update
      and render signals.
*/

use std::local_data;
use glfw;
use log::Log;

static tls_key: local_data::Key<Director> = &local_data::Key;

#[macro_escape]
#[path = "../../shared/log/macros.rs"]
mod macros;

pub trait State
{
  fn load(&mut self);
  fn unload(&mut self)
  { }

  /* Each state must have a unique key that other
   * objects can use to reference it. */
  fn get_key(&self) -> &str;

  /*  Returns true when the event has been captured. If the event is not
      captured, it's set to the next lower state. (A state can capture
      renders, for example, as an optimization -- or updates as a sanity or
      security measure). Rinse and repeat. */
  fn update(&mut self, _delta: f32) -> bool /* dt is in terms of seconds. */
  { false }
  fn render(&mut self) -> bool
  { false }

  /* TODO: Trait inheritance with Input_Listener. */
  fn key_action(&mut self, _key: glfw::Key, _action: glfw::Action, _mods: glfw::Modifiers) -> bool
  { false }
  fn key_char(&mut self, _ch: char) -> bool
  { false }
  fn mouse_action(&mut self, _button: i32, _action: i32, _mods: i32) -> bool
  { false }
  fn mouse_moved(&mut self, _x: f32, _y: f32) -> bool
  { false }
}

pub trait Deferred
{
  fn call(&mut self);
}

pub struct Director
{
  states: ~[@mut State],
  deferreds: ~[@mut Deferred],
}

impl Director
{
  pub fn create()
  {
    let director = Director
    {
      states: ~[],
      deferreds: ~[],
    };

    /* Store the director in task-local storage. (singleton) */
    local_data::set(tls_key, director);
  }

  /* Destroys the director. */
  pub fn destroy()
  {
    let mut director = local_data::pop(tls_key);
    log_assert!(director.is_some());
    let director = director.get_mut_ref();

    while director.states.len() > 0
    { director.states.pop().unload(); }
  }

  /* Accesses the singleton director from task-local storage. */
  pub fn get_mut<T>(handler: &fn(&mut Director) -> T) -> T
  {
    local_data::get_mut(tls_key, 
    |opt|
    {
      match opt
      {
        Some(x) => handler(&mut *x),
        None => log_fail!("Singleton not available")
      }
    })
  }
  pub fn get<T>(handler: &fn(&Director) -> T) -> T
  {
    local_data::get(tls_key, 
    |opt|
    {
      match opt
      {
        Some(x) => handler(&*x),
        None => log_fail!("Singleton not available")
      }
    })
  }

  pub fn push(&mut self, state: @mut State)
  {
    state.load();
    self.states.push(state);
  }

  pub fn pop(&mut self)
  {
    let state = self.states.pop();
    state.unload();
  }

  pub fn unshift(&mut self, state: @mut State)
  {
    state.load();
    self.states.unshift(state);
  }

  pub fn shift(&mut self)
  {
    let state = self.states.shift();
    state.unload();
  }

  pub fn push_deferred(def: @mut Deferred)
  {
    do Director::get_mut |director|
    { director.deferreds.push(def); }
  }

  fn run_deferreds(&mut self)
  {
    for x in self.deferreds.iter()
    { (*x).call(); }
    self.deferreds.clear();
  }

  pub fn remove_if(&mut self, cmp: &fn(@mut State) -> bool)
  {
    if self.states.len() == 0
    { return; }

    let mut x = 0;
    while x < self.states.len()
    {
      if cmp(self.states[x])
      {
        let state = self.states.remove(x);
        state.unload();
        x -= 1;
      }
      x += 1;
    }
  }

  /* Removes the state with the specified key. */
  pub fn pull(&mut self, key: &str)
  {
    let index = do self.states.iter().rposition |state|
    { state.get_key() == key };
    match index
    {
      Some(i) =>
      {
        let state = self.states.remove(i);
        state.unload();
      }
      None => { log_debug!("Invalid state to pull '%s'", key); }
    }
  }

  /* Swaps the state specified by key with the newly
   * specified state. */
  pub fn swap(&mut self, key: &str, state: @mut State)
  {
    let index = do self.states.iter().rposition |st|
    { st.get_key() == key };
    match index
    {
      Some(i) =>
      {
        let old_state = self.states[i];
        old_state.unload();

        state.load();
        self.states[i] = state;
      }
      None => { log_debug!("Invalid state to swap '%s'", key); }
    }
  }

  /** Updating and rendering. **/
  pub fn update(delta: f32)
  {
    let mut states = do Director::get |director|
    { do director.states.map |x| { *x } };
    log_assert!(states.len() > 0);

    for x in states.mut_iter()
    {
      if x.update(delta)
      { break; }
    }
    
    do Director::get_mut |director|
    { director.run_deferreds(); }
  }

  pub fn render()
  {
    let mut states = do Director::get |director|
    { do director.states.map |x| { *x } };
    log_assert!(states.len() > 0);

    for x in states.mut_iter()
    {
      if x.render()
      { break; }
    }

    do Director::get_mut |director|
    { director.run_deferreds(); }
  }

  /** Input handling. **/
  pub fn key_action(key: glfw::Key, action: glfw::Action, mods: glfw::Modifiers)
  {
    let mut states = do Director::get |director|
    { do director.states.map |x| { *x } };
    log_assert!(states.len() > 0);

    for x in states.mut_rev_iter()
    {
      if x.key_action(key, action, mods)
      { break; }
    }

    do Director::get_mut |director|
    { director.run_deferreds(); }
  }

  pub fn key_char(ch: char)
  {
    let mut states = do Director::get |director|
    { do director.states.map |x| { *x } };
    log_assert!(states.len() > 0);

    for x in states.mut_rev_iter()
    {
      if x.key_char(ch)
      { break; }
    }

    do Director::get_mut |director|
    { director.run_deferreds(); }
  }

  pub fn mouse_action(button: i32, action: i32, mods: i32)
  {
    let mut states = do Director::get |director|
    { do director.states.map |x| { *x } };
    log_assert!(states.len() > 0);

    for x in states.mut_rev_iter()
    {
      if x.mouse_action(button, action, mods)
      { break; }
    }

    do Director::get_mut |director|
    { director.run_deferreds(); }
  }

  pub fn mouse_moved(x: f32, y: f32)
  {
    let mut states = do Director::get |director|
    { do director.states.map |x| { *x } };
    log_assert!(states.len() > 0);

    for state in states.mut_rev_iter()
    {
      if state.mouse_moved(x, y)
      { break; }
    }

    do Director::get_mut |director|
    { director.run_deferreds(); }
  }
}

