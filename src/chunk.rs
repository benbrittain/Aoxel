// chunk.rs
// block information for the world


enum Block {
  Space,
  Generic
}


pub struct Chunk {
  land: [[[Block, ..8], ..8], ..8],
  update: bool,
  size: int
}

impl Chunk {
  pub fn new() -> Chunk {
    Chunk {
      land: [[[Generic, ..8], ..8], ..8],
      update: true,
      size: 8
    }
  }
  pub fn get_block(&self, x: int, y: int, z: int) -> Option<Block> {
    match self.land[x][y][z] {
      Space => None,
      _ => Some(self.land[x][y][z])
    }
  }
  pub fn len(&self) -> int {
    self.size
  }
  pub fn reset_update(&mut self) -> () {
    self.update = false;
  }
}

//impl Iterator<Block> for Chunk{
//  fn next(&mut self) -> Option<int> {
//    Some(0)
//  }
//}

