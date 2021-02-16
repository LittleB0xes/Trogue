use crate::entity::{Entity, Tile};
use std::f32::consts::PI;
use tetra::graphics::Color;
use tetra::math::Vec2;

#[derive(Copy, Clone)]
pub enum Direction {
    None,
    North,
    NorthEast,
    NorthWest,
    South,
    SouthEast,
    SouthWest,
    East,
    West,
}

pub enum Action {
    Move,
}

/// Take a geographical direction and return a movment tuple
fn delta_pos(dir: Direction) -> (i32, i32) {
    let delta: (i32, i32);
    match dir {
        Direction::None => delta = (0, 0),
        Direction::North => delta = (0, -1),
        Direction::NorthWest => delta = (-1, -1),
        Direction::NorthEast => delta = (1, -1),
        Direction::South => delta = (0, 1),
        Direction::SouthWest => delta = (-1, 1),
        Direction::SouthEast => delta = (1, 1),
        Direction::East => delta = (1, 0),
        Direction::West => delta = (-1, 0),
    }
    delta
}

/// Take a movment tuple and return a geographical direction
pub fn orientation(dir: (i32, i32)) -> Direction {
    let o: Direction;
    match dir {
        (0, 0) => o = Direction::None,
        (1, 0) => o = Direction::East,
        (-1, 0) => o = Direction::West,
        (0, 1) => o = Direction::South,
        (0, -1) => o = Direction::North,
        (1, 1) => o = Direction::SouthEast,
        (1, -1) => o = Direction::NorthEast,
        (-1, 1) => o = Direction::SouthWest,
        (-1, -1) => o = Direction::NorthWest,
        _ => o = Direction::None,
    }

    o
}

pub fn check_crossable_destination(
    player_x: i32,
    player_y: i32,
    dir: Direction,
    level_map: &Vec<Tile>,
    w: i32,
    h: i32
) -> bool {
    let d = delta_pos(dir);
    let x = player_x + d.0;
    let y = player_y + d.1;

    is_in_map(x, y, w, h) && level_map[(y * w + x) as usize].crossable
}

fn is_in_map(x: i32, y: i32, w: i32, h: i32) -> bool {
    if x>=0 && x < w && y>=0 && y < h {
        true
    } else {
        false
    }
}

pub fn move_entity(entity: &mut Entity, dir: Direction) {
    let delta = delta_pos(dir);
    entity.x += delta.0;
    entity.y += delta.1;
}

/// Pathfinding with A* algorithm
pub fn path_finder(
    x_entity: i32,
    y_entity: i32,
    x_mouse: i32,
    y_mouse: i32,
    level_map: &Vec<Tile>,
    w: i32,
    h: i32,
) -> Vec<Vec2<i32>> {
    #[derive(Copy, Clone, Eq, PartialEq)]
    struct Node {
        id: i32,
        x: i32,
        y: i32,
        f: i32,
        g: i32,
        parent: i32,
    };

    const MAX_CYCLE: i32 = 4000;
    let mut path = Vec::new();

    let mut open_list: Vec<Node> = Vec::new();
    let mut closed_list: Vec<Node> = Vec::new();

    // Add the starting point to the open list
    open_list.push(Node {
        x: x_entity,
        y: y_entity,
        f: 0,
        g: 0,
        id: 0,
        parent: -1,
    });

    let mut id: i32 = 0;

    // Check mouse position
    if is_in_map(x_mouse, y_mouse, w, h)
        && level_map[(x_mouse + y_mouse * w) as usize].crossable
        && level_map[(x_mouse + y_mouse * w) as usize].visited
    {
        let mut cycle = 0;
        while !open_list.is_empty() {
            // Add a depth limit for path finding (to avoid infinite loop)
            //if open_list.len() > (w * h) as usize {
            cycle += 1;
            if cycle > MAX_CYCLE {
                closed_list.clear();
                break;
            }
            let mut current_node = open_list[0];
            let mut best_index: usize = 0;

            // Choose the node with the best score
            for i in 0..open_list.len() {
                if open_list[i].f < current_node.f {
                    current_node = open_list[i];
                    best_index = i;
                }
            }
            current_node.id = id;
            closed_list.push(current_node);

            // Update the node id value
            id += 1;

            // Remove the selected node from the open list
            open_list.remove(best_index);

            // If current is the goal, the break
            if current_node.x == x_mouse && current_node.y == y_mouse {
                break;
            }

            // create children list of cuurent node
            let mut children: Vec<Vec2<i32>> = Vec::new();
            for c in [
                (1, 0),
                (-1, 0),
                (0, 1),
                (0, -1),
                (1, 1),
                (1, -1),
                (-1, 1),
                (-1, -1),
            ]
            .iter()
            {
                // keep only node in the map and crossable
                if c.0 + current_node.x >= 0
                    && c.0 + current_node.x < w
                    && c.1 + current_node.y >= 0
                    && c.1 + current_node.y < h
                    && level_map[(c.0 + current_node.x + (c.1 + current_node.y) * w) as usize]
                        .crossable
                    && level_map[(c.0 + current_node.x + (c.1 + current_node.y) * w) as usize]
                        .visited
                {
                    children.push(Vec2::new(c.0 + current_node.x, c.1 + current_node.y));
                }
            }

            for c in children.iter() {
                // Zap node who are already in closed list
                if closed_list.iter().any(|&n| n.x == c.x && n.y == c.y) {
                    continue;
                }
                let g = current_node.g + 1;
                let h = distance(c.x, c.y, x_mouse, y_mouse);
                let f = g + h;

                let mut out = false;

                // Just keep the best node if it's already in open list
                for op in open_list.iter() {
                    if op.x == c.x && op.y == c.y && op.g < g {
                        out = true;
                    }
                }
                if out {
                    continue;
                }

                open_list.push(Node {
                    x: c.x,
                    y: c.y,
                    f: f,
                    g: g,
                    id: -1,
                    parent: current_node.id,
                });
            }
        }
    }

    // Now, take the best way
    // Go from child to parent until reach the starting point
    if closed_list.len() != 0 {
        let mut node = closed_list[closed_list.len() - 1];
        path.push(Vec2::new(node.x, node.y));
        while node.parent != -1 {
            for o in closed_list.iter() {
                if node.parent == o.id {
                    path.push(Vec2::new(o.x, o.y));
                    node = Node {
                        id: o.id,
                        x: o.x,
                        y: o.y,
                        f: o.f,
                        g: o.g,
                        parent: o.parent,
                    };
                }
            }
        }
    }

    path
}

fn distance(x1: i32, y1: i32, x2: i32, y2: i32) -> i32 {
    (x1 - x2).pow(2) + (y1 - y2).pow(2)
}

/// Color modification for visited tile
pub fn visited_color(color: Color) -> (u8, u8, u8) {
    let f = 0.5;
    let r = (color.r * 255.0 * f) as u8;
    let g = (color.g * 255.0 * f) as u8;
    let b = (color.b * 255.0 * f) as u8;

    (r, g, b)
}

// Simple raycasting fov with range view
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
