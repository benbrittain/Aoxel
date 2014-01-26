// main.rs
// Ben Brittain
//

#[feature(globs)];

extern mod glfw;
extern mod gl;
extern mod green;
extern mod cgmath;
extern mod noise;

extern mod extra;

// standard libraries and such
use std::libc;
use std::gc;
use extra::arc::*;

// parts of this project
use window::Window;
use chunk::*;
use renderer::*;
use world::*;
use camera::*;
use event::*;
use event::EventHandler;

mod window;
mod event;
mod chunk;
mod renderer;
mod world;
mod camera;

fn main() {
  println!("Starting Aoxel...");

  do Window::spawn(~"Aoxel", 800, 600, false) | main_window | {

    main_window.run_loop(| window |  {

      let mut world: World = World::new();

      window.set_world(world);



    });
  }
}
//    // Setup Event handlers
//    let event_handler = EventHandler::new();
//
//    window.set_key_callback(~KeyCallback::new(event_handler.get_collector()));
//
//    // Initialize renderer
//    let mut renderer:Renderer = Renderer::new();
//    // Create the World which manages Chunks
////    renderer.add_map(world);
//
//    // Create the Camera
//    let mut camera: Camera = Camera::new();
//
//
//
//    while !window.should_close() {
//      glfw::poll_events();
//
//      gl::ClearColor(0.1, 0.1, 0.1, 0.1);
//      gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
//
//      event_handler.poll_events(camera);
//
//      // TODO callback!
////      match window.get_cursor_pos() {
////        (x,y) => {
////          let dx = x - 600.0/2.0;
////          let dy = y - 800.0/2.0;
////          camera.update(x as f32, y as f32);
////          println!("{},{}", x, y);
////        }
////      }
//      renderer.set_world_to_camera(camera.view());
//      renderer.update();
//
//      // render
// //     renderer.update(&mut chunk);
//      window.swap_buffers();
//    }
//
//  }
//}
//
////struct CursorPosContext;
////impl glfw::CursorPosCallback for CursorPosContext {
////  fn call (&self, window: &glfw::Window, x_pos: f64, y_pos: f64) {
////    camera.update();
////    println!("Cursor: ({},{})", x_pos, y_pos);
////  }
////}
////
//
//struct KeyCallback {
//  collector: RWArc<~[Event]>
//}
//
//
//impl KeyCallback {
//  pub fn new(collector: RWArc<~[Event]>) -> KeyCallback {
//    KeyCallback {
//      collector: collector
//    }
//  }
//}
//
//struct Keycallback;
//impl glfw::KeyCallback for KeyCallback {
//  fn call (&self, window: &glfw::Window, key: glfw::Key,
//          _scancode: libc::c_int, action: glfw::Action,
//          _mods: glfw::Modifiers) {
//    match (key, action) {
//      (glfw::KeyEscape, glfw::Press) => window.set_should_close(true),
//
//      (_, glfw::Press) => self.collector.write(|c|
//                                               c.push(event::KeyPressed(key))),
//
//      (_, glfw::Release) => self.collector.write(|c|
//                                               c.push(event::KeyReleased(key))),
//      _ => println!("{}", key.to_str())
//    }
//
//  }
//}
//
//
#[start]
fn start(argc: int, argv: **u8) -> int {
  //TODO initialize more advanced logging
  do green::start(argc, argv) {
    main();
  }
}
