use crate::entity::{Tile, Entity};
use std::f32::consts::PI;
use tetra::math::Vec2;

#[derive(Copy, Clone)]
pub enum Direction {
    None,
    North,
    South,
    East,
    West
}

pub enum Action {
    Move,
}


fn delta_pos(dir : Direction) -> (i32,i32) {
    let delta: (i32, i32);
    match dir {
        Direction::None => delta = (0,0),
        Direction::North => delta = (0,-1),
        Direction::South => delta = (0,1),
        Direction::East => delta = (-1,0),
        Direction::West => delta = (1,0),
    }
    delta
}

pub fn check_crossable_destination(player_x: i32, player_y: i32, dir: Direction, level_map: &Vec<Tile>, w: i32) -> bool {
    let d = delta_pos(dir);
    let x = player_x + d.0;
    let y = player_y + d.1;

    level_map[(y * w + x) as usize].crossable
}

pub fn move_entity(entity: &mut Entity, dir: Direction) {
    let delta = delta_pos(dir);
    entity.x += delta.0;
    entity.y += delta.1;
}

pub fn path_finder(x_entity: i32, y_entity: i32, x_mouse: i32, y_mouse: i32, level_map: &Vec<Tile>, w: i32, h: i32) -> Vec<Vec2<i32>>{
    let mut path = Vec::new();

        let a = (y_entity - y_mouse) as f32 / (x_entity - x_mouse) as f32;
        let b = y_entity as f32 - a * x_entity as f32;

        for i in 0..((w*h) as usize) {
            let x = i as i32 % w;
            let y = i as i32 / w;
            if y == ((a * x as f32 + b).floor()) as i32 || y == ((a * x as f32 + b).floor()) as i32 {
                path.push(Vec2::new(x, y));
            }
        }

    path
}

pub fn fov(
    x_entity: i32,
    y_entity: i32,
    range: i32,
    level_map: &mut Vec<Tile>,
    w: i32,
    h: i32,
) -> Vec<[i32; 2]> {
    // Create vec of tiles in fov
    let mut in_fov_tile: Vec<[i32; 2]> = Vec::new();

    // Player's tile allways visible
    in_fov_tile.push([x_entity, y_entity]);

    // Initialize all tile to unsee
    for i in 0..w {
        for j in 0..h {
            level_map[(i + j * w) as usize].visible = false;
        }
    }

    for a in 0..360 {
        // Set normalize direction vector
        let x = ((a as f32) * PI / 180.0).cos();
        let y = ((a as f32) * PI / 180.0).sin();

        // Player position (center)
        let mut dx = (x_entity as f32) + 0.5;
        let mut dy = (y_entity as f32) + 0.5;

        for _i in 0..range {
            // break if out of map
            if dx >= w as f32 || dx < 0.0 || dy < 0.0 || dy >= h as f32 {
                break;
            }

            // index af tile
            let z = (dx.trunc() as i32) + (dy.trunc() as i32) * w;

            level_map[z as usize].visible = true;
            level_map[z as usize].visited = true;

            
            // Add tile in fov
            in_fov_tile.push([z % w, z / w]);

            if level_map[z as usize].see_through == false && z != x_entity + y_entity * w
            // For the Door visual effect when Player is on a Door
            {
                break;
            }
            dx += x;
            dy += y;
        }
    }

    in_fov_tile
}