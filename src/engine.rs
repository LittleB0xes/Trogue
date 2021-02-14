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

    #[derive(Copy, Clone, Eq, PartialEq)]
    struct Node {
        x: i32,
        y: i32,
        f: i32
    };
    let mut path = Vec::new();
    
    let mut open_list: Vec<Node> = Vec::new();
    let mut closed_list: Vec<Node> = Vec::new();

    // Add the starting point to the open list
    open_list.push(Node{x: x_entity, y: y_entity, f: 0});

    while !open_list.is_empty() {
        //println!("open list {}, closed_list {}", open_list.len(), closed_list.len());
        let mut current_node = open_list[0];// Node{x: x_entity, y:y_entity, f: 50000};
        let mut best_index: usize = 0;
        for i in 0..open_list.len() {
            if open_list[i].f < current_node.f {
                current_node = open_list[i];
                best_index = i;
            }
        }
        closed_list.push(current_node);
        open_list.remove(best_index);

        // If current                    is the goal, the break
        if current_node.x == x_mouse && current_node.y == y_mouse {
            //println!("OOOOOOOOOOOOOOOOKKKKKKKKKKKKKKKK");
            continue;
            //break;
            //done = true;
        }

        // create children list
        let mut children: Vec<Vec2<i32>> = Vec::new();
        for c in [(1,0), (-1, 0), (0, 1), (0, -1), (1,1), (1, -1), (-1, 1), (-1, -1)].iter() {
            if c.0 + current_node.x >= 0 && c.0 + current_node.x < w && c.1 + current_node.y >= 0 && c.1 + current_node.y < h && level_map[(c.0 + current_node.x + (c.1 + current_node.y) * w) as usize].crossable {

                children.push(Vec2::new(c.0 + current_node.x, c.1 + current_node.y));
            }
        }

        for c in children.iter() {
            //println!("children {}", children.len());

            //if closed_list.iter().any(|&n| n.x == c.x) && closed_list.iter().any(|n| n.y == c.y) {
            //    continue;
            //}
            let g = (current_node.x - x_entity).pow(2) + (current_node.y - y_entity).pow(2) + 2 ;//+ (c.x - x_entity).pow(2) + (c.y - y_entity).pow(2);
            let h = (c.x - x_mouse).pow(2) + (c.y - y_mouse).pow(2);
            let f = g + h;

            let mut out = false;
            //for cl in closed_list.iter() {
            //    if cl.x == c.x && cl.y == c.y {
            //        out = true;
            //        continue;
            //    }
            //}
            if closed_list.iter().any(|&n| n.x == c.x && n.y == c.y) {
                continue;
            }
            for op in open_list.iter() {
                let opg = (op.x - x_entity).pow(2) + (op.y - y_entity).pow(2);
                if op.x == c.x && op.y == c.y && opg < g {
                    out = true;
                }
            }
            if out {
                break;
            }
            open_list.push(Node{x: c.x, y: c.y, f: f});
            


        }




    }

    for node in closed_list.iter() {
        path.push(Vec2::new(node.x, node.y));
    }
        //let a = (y_entity - y_mouse) as f32 / (x_entity - x_mouse) as f32;
        //let b = y_entity as f32 - a * x_entity as f32;

        //for i in 0..((w*h) as usize) {
        //    let x = i as i32 % w;
        //    let y = i as i32 / w;
        //    if y == ((a * x as f32 + b).floor()) as i32 || y == ((a * x as f32 + b).floor()) as i32 {
        //        path.push(Vec2::new(x, y));
        //    }
        //}            
        //println!("path {}", path.len());
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