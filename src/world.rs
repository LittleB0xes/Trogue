use crate::entity::{Entity, EntityType,ItemType, Tile};
use rand::prelude::*;

pub enum GenerationType {
    Random,
    Cave,
    Floor,
}

pub fn world_genration(w: i32, h: i32, gen_type: GenerationType) -> Vec<Tile> {
    let mut level = Vec::new();
    match gen_type {
        GenerationType::Random => level = random_level(w, h),
        GenerationType::Cave => level = cave_level(w, h),
        GenerationType::Floor => level = floor_level(w, h),
    }

    level
}

pub fn random_level(w: i32, h: i32) -> Vec<Tile> {
    let mut rng = rand::thread_rng();
    let mut level_map = Vec::new();
    for i in 0..(w * h) as usize {
        let alea: u8 = rng.gen_range(0..=100);
        let tile: Tile;

        if alea < 1 {
            tile = Tile::new(i as i32 % w, i as i32 / w, ItemType::StoneWall);
        } else {
            tile = Tile::new(i as i32 % w, i as i32 / w, ItemType::StoneFloor);
        }
        level_map.push(tile);
    }

    level_map
}

pub fn cave_level(w: i32, h: i32) -> Vec<Tile> {
    let mut rng = rand::thread_rng();

    // Sector division
    // 16/12/5 => Labyrinthe, peu de salles, quelques zone isolées
    // 8/6/12 => Grandes grottes communicantes
    let h_cells: usize = 8;
    let v_cells: usize = 6;
    let mut cycle = 12;
    let width = w / h_cells as i32;
    let height = h / v_cells as i32;

    //
    let mut level_map = Vec::new();
    let mut draft = Vec::new();

    //Fill map with Wall
    for _i in 0..(w * h) as usize {
        draft.push(ItemType::MudWall);
    }

    // seeding each sector

    for i in 0..h_cells {
        for j in 0..v_cells {
            let x = i as i32 * width + rng.gen_range(1..width);
            let y = j as i32 * height + rng.gen_range(1..height);
            draft[(x + y * w) as usize] = ItemType::StoneFloor;
        }
    }

    let mut temp = Vec::new();
    temp = draft.clone();

    // Let growing

    // Direction of the walker
    let direction: [i32; 4] = [-1, 1, -w, w];

    // Walk cycle
    while cycle > 0 {
        for i in 0..(w * h) as usize {
            match draft[i] {
                ItemType::StoneFloor => {
                    temp[i] = ItemType::StoneFloor;
                    let new_index = (i as i32) + direction[rng.gen_range(0..4)];
                    let x = new_index % w;
                    let y = new_index / w;
                    if x >= 0 && x < w && y >= 0 && y < h {
                        temp[new_index as usize] = ItemType::StoneFloor;
                    }
                }
                _ => {}
            }
        }

        draft = temp.clone();

        cycle -= 1;
    }

    for i in 0..(w * h) as usize {
        level_map.push(Tile::new(i as i32 % w, i as i32 / w, draft[i]));
    }

    level_map
}

pub fn floor_level(w: i32, h: i32) -> Vec<Tile> {
    let mut rng = rand::thread_rng();
    let mut level_map = Vec::new();

    // First, the Corridors

    // Choose starting point
    let x: i32 = rng.gen_range(5..(w - 5));
    let y: i32 = rng.gen_range(5..(h - 5));

    let dir = [-1, 1, -w, w][rng.gen_range(0..4)];

    let length = rng.gen_range(5..10);

    // Choose direction length

    // Draw corridoe

    level_map
}

pub fn spawn_npc(level_map: &Vec<Tile>, w: i32, h: i32) -> Vec<Entity> {
    let mut npc = Vec::new();
    let mut rng = rand::thread_rng();
    for _i in 0..20 {
        let index = rng.gen_range(0..(w*h));
        if level_map[index as usize].crossable {
            npc.push(Entity::new(index % w, index / w, EntityType::Zombie));
        }

    }

    npc
}