extern mod glfw;

use extra::arc;
use camera::*;


use glfw::Key;

#[deriving(ToStr)]
pub enum Event {
  CursorPos(f64, f64),
  KeyPressed(glfw::Key),
  KeyReleased(glfw::Key)
}

pub struct EventHandler {
  event_collector: arc::RWArc<~[Event]>
}

impl EventHandler {
  pub fn new() -> EventHandler {
    EventHandler {
      event_collector: arc::RWArc::new(~[])
    }
  }

//  pub fn link_cb(&self, cb: |arc::RWArc<~[Event]>| -> ()) {
//    self.event_collector.clone()
//  }

  pub fn get_collector(&self) -> arc::RWArc<~[Event]> {
    self.event_collector.clone()
  }

  pub fn poll_events(&self, actions: |event: &Event| ->()){
    self.event_collector.read(
      |event_vec| for event in event_vec.iter()  {
        actions(event)
      });
    self.event_collector.write(
      |event_vec| event_vec.clear());
  }
}
