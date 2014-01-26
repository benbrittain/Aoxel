// camera.rs
// Control FP camera

extern mod glfw;

use cgmath::*;
use cgmath::vector::*;
use cgmath::matrix::*;
use cgmath::point::*;

use std::num;
use std::libc;

pub struct Camera {
  position: Point3<f32>,
  angles: Vec2<f32>
}

impl Camera {
  pub fn new() -> Camera {
    Camera {
      position: Point3::new(75.0 as f32, 75.0 as f32, 75.0 as f32),
      angles:   Vec2::new(0.0 as f32, 0.0 as f32)
    }
  }

  pub fn update(&mut self, x: f32, y: f32) -> () {
    self.angles = Vec2::new(x, y);
  }

  pub fn view(&self) -> Mat4<f32> {
    let mut look_at: Vec3<f32> = Vec3::new(0.0 as f32, 0.0, 0.0);
    look_at.x = num::sin(self.angles.x) * num::cos(self.angles.y);
    look_at.y = num::sin(self.angles.y);
    look_at.z = num::cos(self.angles.x) * num::cos(self.angles.y);
    Mat4::look_at(&self.position,
//                  &Point3::new(0.0 as f32, 0.0, 0.0),
//                  &Vec3::new(0.0 as f32, 0.0, 1.0))
                  &Point::from_vec(&look_at.mul(&self.position.to_vec())),
                  &Vec3::new(0.0 as f32, 0.0, 1.0))
  }
}

