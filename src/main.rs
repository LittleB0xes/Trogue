use tetra::graphics::Color;
use tetra::input::{self, Key};
use tetra::math::Vec2;
use tetra::{time, Context, ContextBuilder, Event, State};
use rand::prelude::*;

mod entity;
use entity::{Entity, EntityType, Tile};

mod world;
use world::*;

mod engine;
use engine::{Action, Direction};

mod grl;
use grl::Terminal;

const VIEW_WIDTH: i32 = 64;
const VIEW_HEIGHT: i32 = 48;
const UI_SIZE: i32 = 20;
const HEIGHT: i32 = VIEW_HEIGHT;
const WIDTH: i32 = VIEW_WIDTH + UI_SIZE;
const CELL_SIZE: i32 = 16;

struct Mouse {
    x: i32,
    y: i32,
    active: bool,
    cell_moved: bool,
    clicked: bool,
}

struct GameState {
    map_width: i32,
    map_height: i32,
    mouse: Mouse,
    terminal: Terminal,
    floor_map: Vec<Tile>,
    npc_list: Vec<Entity>,
    in_fov: Vec<[i32;2]>,
    player: Entity,
    path: Vec<Vec2<i32>>,
    auto_walk: bool,
    turn: u32,
    player_turn: bool,
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        let w = VIEW_WIDTH;
        let h = VIEW_HEIGHT;
        let map = world::world_genration(w, h, GenerationType::Cave);
        let npc = world::spawn_npc(&map, w, h);
        Ok(GameState {
            map_width: w,
            map_height: h,
            mouse: Mouse {
                x: 1,
                y: 1,
                active: false,
                cell_moved: false,
                clicked: false,
            },
            terminal: Terminal::new(ctx, WIDTH, HEIGHT, CELL_SIZE, CELL_SIZE),
            player: Entity::new(10, 10, EntityType::Player),
            path: Vec::new(),
            auto_walk: false,
            floor_map: map,
            npc_list: npc,
            in_fov: Vec::new(),
            turn: 0,
            player_turn: true,
        })
    }
    fn action_manager(&mut self, action: Action, dir: Direction) {
        match action {
            Action::Move => {
                // Switch mouse to inactive if key down
                self.mouse.active = false;
                if engine::check_crossable_destination(
                    self.player.x,
                    self.player.y,
                    dir,
                    &self.floor_map,
                    self.map_width,
                    self.map_height
                ) {
                    engine::move_entity(&mut self.player, dir);

                    self.player_turn = false;
                }
            }
        }
    }
}

impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        // Cornflower blue, as is tradition
        self.terminal.clear(ctx);

        self.terminal.layer(0);
        // Map display

        for tile in self.floor_map.iter() {
            if tile.visible {
                self.terminal.fg_color(tile.fg_color);
                self.terminal.bg_color(tile.bg_color);
                self.terminal.put(UI_SIZE + tile.x, tile.y, tile.glyph);
            } else if tile.visited {
                let fg = engine::visited_color(tile.fg_color);
                let bg = engine::visited_color(tile.bg_color);
                self.terminal.fg_color(Color::rgb8(fg.0, fg.1, fg.2));
                self.terminal.bg_color(Color::rgb8(bg.0, bg.1, bg.2));
                self.terminal.put(UI_SIZE + tile.x, tile.y, tile.glyph);
            }
        }

        for n in self.npc_list.iter() {
            if self.in_fov.iter().any(|&t| t[0] == n.x && t[1] == n.y) {
                let npc_index = (n.y * self.map_width + n.x) as usize;
                self.terminal
                    .bg_color(self.floor_map[npc_index as usize].bg_color);
                self.terminal.fg_color(n.fg_color);
                self.terminal
                    .put(UI_SIZE + n.x, n.y, n.glyph);

            }
        }

        // Player display
        let player_index = (self.player.y * self.map_width + self.player.x) as usize;
        self.terminal
            .bg_color(self.floor_map[player_index].bg_color);
        self.terminal.fg_color(self.player.fg_color);
        self.terminal
            .put(UI_SIZE + self.player.x, self.player.y, self.player.glyph);

        self.terminal.bg_color(Color::rgb8(0, 0, 0));
        self.terminal.print(
            0,
            0,
            format!("Position {} - {}", self.player.x, self.player.y),
        );
        self.terminal
            .print(0, 1, format!("Mouse {} - {}", self.mouse.x, self.mouse.y));
        self.terminal.print(0, 2, format!("Turn {}", self.turn));
        self.terminal
            .print(0, 3, format!("FPS {}", time::get_fps(ctx) as i32));

        self.terminal.layer(1);
        // Draw path
        if self.mouse.active {
            self.terminal.fg_color(Color::rgba8(255, 255, 0, 50));
            if self.mouse.cell_moved {
                self.path = engine::path_finder(
                    self.player.x,
                    self.player.y,
                    self.mouse.x,
                    self.mouse.y,
                    &self.floor_map,
                    self.map_width,
                    self.map_height,
                );
                self.path.reverse();

                // Remove starting point from the path
                if !self.path.is_empty() {

                    self.path.remove(0);
                }
            }
            for step in self.path.iter() {
                self.terminal.put(UI_SIZE + step.x, step.y, 219);
            }
        }
        // Mouse Display
        if self.mouse.active {
            self.terminal.fg_color(Color::rgba8(255, 255, 0, 100));
            self.terminal.put(UI_SIZE + self.mouse.x, self.mouse.y, 219);
        }

        self.terminal.bg_color(Color::rgb8(0, 0, 0));
        self.terminal.refresh(ctx);
        Ok(())
    }

    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        if self.player_turn {
            self.in_fov = engine::fov(
                self.player.x,
                self.player.y,
                10,
                &mut self.floor_map,
                self.map_width,
                self.map_height,
            );

            if !self.path.is_empty() && self.mouse.clicked {
                self.auto_walk = true;
            } else if self.path.is_empty() {
                self.auto_walk = false;
            }

            if self.auto_walk {
                let dir = (
                    self.path[0].x - self.player.x,
                    self.path[0].y - self.player.y,
                );
                self.action_manager(Action::Move, engine::orientation(dir));
                self.path.remove(0);
            }

            if input::is_key_pressed(ctx, Key::Left) {
                self.action_manager(Action::Move, Direction::West);
            } else if input::is_key_pressed(ctx, Key::Right) {
                self.action_manager(Action::Move, Direction::East);
            } else if input::is_key_pressed(ctx, Key::Up) {
                self.action_manager(Action::Move, Direction::North);
            } else if input::is_key_pressed(ctx, Key::Down) {
                self.action_manager(Action::Move, Direction::South);
            }


        }
        else {
            //Monster turn
            let mut rng = rand::thread_rng();
            for npc in self.npc_list.iter_mut() {
                let x: i32 = rng.gen_range(-1..=1);
                let y: i32 = rng.gen_range(-1..=1);
                if engine::check_crossable_destination(npc.x, npc.y, engine::orientation((x,y)), &self.floor_map, self.map_width, self.map_height) {

                    engine::move_entity(npc, engine::orientation((x,y)));
                }
                
            }

            self.player_turn = true;

            self.turn += 1;
        }
        if input::is_key_pressed(ctx, Key::Space) {
            self.floor_map =
                world::world_genration(self.map_width, self.map_height, GenerationType::Cave);
        }
        Ok(())
    }
    fn event(&mut self, _: &mut Context, event: Event) -> tetra::Result {
        self.mouse.clicked = false;
        match event {
            Event::MouseMoved { position, .. } => {
                let new_pos_x = position.x as i32 / CELL_SIZE - UI_SIZE;
                let new_pos_y = position.y as i32 / CELL_SIZE;
                self.mouse.active = true;
                if self.mouse.x != new_pos_x || self.mouse.y != new_pos_y {
                    self.mouse.cell_moved = true;
                    self.mouse.x = position.x as i32 / CELL_SIZE - UI_SIZE;
                    self.mouse.y = position.y as i32 / CELL_SIZE;
                } else {
                    self.mouse.cell_moved = false;
                }
            }
            Event::MouseButtonPressed { button } => {
                self.mouse.clicked = true;
            }
            Event::MouseButtonReleased { button } => {
                self.mouse.clicked = false;
            }
            _ => {}
        }
        Ok(())
    }
}

fn main() -> tetra::Result {
    ContextBuilder::new("Hello, world!", WIDTH * CELL_SIZE, HEIGHT * CELL_SIZE)
        .quit_on_escape(true)
        .build()?
        .run(GameState::new)
}
