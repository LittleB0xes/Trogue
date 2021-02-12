use rand::prelude::*;
use crate::entity::{Tile, ItemType};

pub enum GenerationType {
  Random,
  Cave,
}

pub fn world_genration(w: i32, h: i32, gen_type: GenerationType) -> Vec<Tile> {
  let mut level = Vec::new();
  match gen_type {
    GenerationType::Random => level = random_level(w, h),
    GenerationType::Cave => level = cave_level(w, h),
  }

  level
}


pub fn random_level(w: i32, h: i32) -> Vec<Tile> {
  let mut rng = rand::thread_rng();
  let mut level_map = Vec::new();
  for i in 0..(w * h) as usize {
    let alea: u8 = rng.gen_range(0..=100);
    let tile: Tile;

    if alea < 10 {
      tile = Tile::new(i as i32 % w, i as i32 / w, ItemType::StoneWall);
    }
    else {
      tile = Tile::new(i as i32 % w, i as i32 / w, ItemType::StoneFloor);
    }
    level_map.push(tile);
  }

  level_map
}

pub fn cave_level(w: i32, h: i32) -> Vec<Tile> {
  let mut level_map = Vec::new();
  let mut draft: Vec<ItemType> = Vec::new();
  

  // Fill map with Wall
  for _i in 0..(w * h) as usize {
    draft.push(ItemType::MudWall);
  }

  // split the map in sector


  // seeding each sector


  // Let growing

  for i in  0..(w * h) as usize {
    level_map.push(Tile::new(i as i32 % w, i as i32 / w, draft[i]));

  }

  level_map
}