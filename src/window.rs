// External libraries
extern mod gl;
extern mod glfw;
extern mod extra;

use std::libc;
use extra::arc::*;
use world::World;
// Project components
mod world;
mod renderer;
mod camera;
mod event;
mod chunk;

pub struct Window {
  window: glfw::Window,
  fullscreen: bool,
  height: int,
  width: int,
  title: ~str,
  camera: camera::Camera,
  event_handler: event::EventHandler,
  map_renderer: renderer::Renderer,

  world: Option<World>

}

impl Window {
  pub fn spawn(title: ~str, width: int, height: int,
               fullscreen: bool, callback: proc(&mut Window)) {

    // Start glfw
    do glfw::start {
      glfw::window_hint::context_version_major(3);
      glfw::window_hint::context_version_minor(2);
      glfw::window_hint::opengl_profile(glfw::OpenGlCoreProfile);

      // Create the visible window
      let window = match fullscreen {
        false => glfw::Window::create(width as u32,
                                        height as u32,
                                        title, glfw::Windowed),
        true => {
          let monitor = glfw::Monitor::get_primary().unwrap();
          let (w, h) = monitor.get_physical_size();
          glfw::Window::create(w as u32,
                               h as u32,
                               title, glfw::FullScreen(monitor))
        }
      }.unwrap();


      // Set the Context to the glfw window
      window.make_context_current();
      gl::load_with(glfw::get_proc_address);

      // Setup the event handlers
      let event_handler = event::EventHandler::new();
      window.set_key_callback(~KeyCallback::new(event_handler.get_collector()));
      window.set_cursor_pos_callback(~CursorPosCallback::new(event_handler.get_collector()));

      let map_renderer = renderer::Renderer::new();

      let camera = camera::Camera::new();

      // Create the Window object
      let mut c_window: Window =
        Window {
          window: window,
          fullscreen: fullscreen,
          title: title,
          height: height,
          width: width,
          camera: camera,
          event_handler: event_handler,
          map_renderer: map_renderer,

          world: None
        };

      callback(&mut c_window)
    }
  }

  pub fn set_world(&mut self, world: World) {
    self.world = Some(world);
  }

  pub fn run_loop(&mut self, cb: |&mut Window| -> ()) {
    while !self.window.should_close() {

      glfw::poll_events();

      gl::ClearColor(0.1, 0.1, 0.1, 0.1);
      gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

      // This is where all the events in main occur
      cb(self);

      self.event_handler.poll_events(|event: &event::Event | {
        match *event {
          event::KeyPressed(key) => println!("{}", key.to_str()),
          event::CursorPos(x, y) => self.camera.update(x as f32, y as f32),
          _ => ()
        }
      });

      self.map_renderer.update();
      self.map_renderer.set_world_to_camera(self.camera.view());

      self.window.swap_buffers();

    }
  }
}

// TODO evaluate how putting this glfw callbacks here is working
// too tightly coupled with the event module
// More intelligently design this?

// Everything related to key presses
struct KeyCallback {
  collector: RWArc<~[event::Event]>
}

impl KeyCallback {
  pub fn new(collector: RWArc<~[event::Event]>) -> KeyCallback {
    KeyCallback {
      collector: collector
    }
  }
}

impl glfw::KeyCallback for KeyCallback {
  fn call (&self, window: &glfw::Window, key: glfw::Key,
           _scancode: libc::c_int, action: glfw::Action,
           _mods: glfw::Modifiers) {
    match (key, action) {
      // Emergency Escape!!!
      (glfw::KeyEscape, glfw::Press) => window.set_should_close(true),

      (_, glfw::Press) => self.collector.write(|c|
                                               c.push(event::KeyPressed(key))),

      (_, glfw::Release) => self.collector.write(|c|
                                               c.push(event::KeyReleased(key))),
      _ => println!("{}", key.to_str())
    }
  }
}

// Everything related to Cursor Position

struct CursorPosCallback {
  collector: RWArc<~[event::Event]>
}

impl CursorPosCallback {
  pub fn new(collector: RWArc<~[event::Event]>) -> CursorPosCallback {
    CursorPosCallback {
      collector: collector
    }
  }
}

impl glfw::CursorPosCallback for CursorPosCallback {
  fn call (&self, window: &glfw::Window, x_pos: f64, y_pos: f64){
    self.collector.write( |c|
                          c.push(event::CursorPos(x_pos, y_pos)));
  }
}
//    match (x_pos, y_pos) {
//      // Emergency Escape!!!
//      (, glfw::Press) => window.set_should_close(true),
//
//      (_, glfw::Press) => self.collector.write(|c|
//                                               c.push(event::KeyPressed(key))),
//
//      (_, glfw::Release) => self.collector.write(|c|
//                                               c.push(event::KeyReleased(key))),
//      _ => println!("{}", key.to_str())
//    }
//  }
//}
//
