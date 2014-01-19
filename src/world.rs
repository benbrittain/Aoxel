// world.rs
// block information for the world


enum Block {
  Generic
}


pub struct World {
  land: [[[Block, ..5], ..5], ..5]
}

impl World {
  pub fn new() -> World {
    World { land: [[[Generic, ..5], ..5], ..5] }
  }
}
