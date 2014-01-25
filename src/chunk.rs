// chunk.rs
// block information for the world
use std::rand;
use std::rand::*;

pub enum Block {
  Space = 0,
  Generic,
  Generic2,
  Generic3
}

pub struct Chunk {
  land: [[[Block, ..16], ..16], ..16],
  update: bool,
  size: int
}

impl Chunk {
  pub fn new() -> Chunk {
    Chunk {
      land: [[[Generic, ..16], ..16], ..16],
      update: true,
      size: 16
    }
  }

  pub fn new_with_random() -> Chunk {
    let mut mut_land: [[[Block, ..16], ..16], ..16] =  [[[Generic, ..16], ..16], ..16];
    for x in range (0, 16){
      for y in range (0, 16) {
        for z in range (0, 16) {
          let mut rng = rand::task_rng();
          mut_land[x][y][z] = match rng.gen_range(0,4) {
            0 => Space,
            1 => Generic,
            2 => Generic2,
            3 => Generic3,
            _ => Space
          };
        }
      }
    }
    Chunk {
      land: mut_land,
      update: true,
      size: 16
    }
  }

  pub fn get_block(&self, x: int, y: int, z: int) -> Option<Block> {
    if (x < 0 || y < 0 || z < 0) {
      return None
    }
    if (x >= 16 || y >= 16 || z >= 16) {
      return None
    }
    match self.land[x][y][z] {
      Space => None,
      _ => Some(self.land[x][y][z])
    }
  }

  pub fn len(&self) -> int {
    self.size
  }

  // Mutable functions
  pub fn reset_update(&mut self) -> () {
    self.update = false;
  }

  pub fn set_block(&mut self, block: Block, x: int, y: int, z: int) -> () {
    self.land[x][y][z] = block;
  }
}

//impl Iterator<Block> for Chunk{
//  fn next(&mut self) -> Option<int> {
//    Some(0)
//  }
//}

