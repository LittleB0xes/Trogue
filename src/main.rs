use tetra::{Context, ContextBuilder, State, Event};
use tetra::graphics::Color;
use tetra::input::{self, Key};

mod entity;
use entity::{Entity, Tile, ItemType};

mod world;
use world::*;

mod engine;
use engine::{Direction, Action};

mod grl;
use grl::Terminal;


const VIEW_WIDTH: i32 = 40;
const VIEW_HEIGHT: i32 = 25;
const UI_SIZE: i32 = 20;
const HEIGHT: i32 = VIEW_HEIGHT;
const WIDTH: i32 = VIEW_WIDTH + UI_SIZE;
const CELL_SIZE: i32 = 24;

struct Mouse {
    x: i32,
    y: i32,
    active: bool,
}

struct GameState {
    map_width: i32,
    map_height: i32,
    mouse: Mouse,
    terminal: Terminal,
    floor_map: Vec<Tile>,
    player: Entity,

    
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        let w = VIEW_WIDTH;
        let h = VIEW_HEIGHT;
        Ok(GameState {
            map_width: w,
            map_height: h,
            mouse: Mouse{x: 1, y: 1, active: false},
            terminal: Terminal::new(ctx, WIDTH, HEIGHT, CELL_SIZE, CELL_SIZE),
            player: Entity::new(10, 10, 64),
            floor_map: world::world_genration(w, h, GenerationType::Random),


        })
    }
    fn action_manager(&mut self, action: Action, dir: Direction) {
        match action {
            Action::Move => {
                // Switch mouse to inactive if key down
                self.mouse.active = false;
                if engine::check_crossable_destination(self.player.x, self.player.y, dir, &self.floor_map, self.map_width) {
                    engine::move_entity(&mut self.player, dir);
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

            }
        }

        // Player display
        let player_index = (self.player.y * self.map_width + self.player.x) as usize; 
        self.terminal.bg_color(self.floor_map[player_index].bg_color);
        self.terminal.fg_color(self.player.fg_color);
        self.terminal.put(UI_SIZE + self.player.x, self.player.y, self.player.glyph);

        self.terminal.bg_color(Color::rgb8(0,0,0));
        self.terminal.print(0,0, format!("Position {} - {}", self.player.x, self.player.y));
        self.terminal.print(0,1, format!("Mouse {} - {}", self.mouse.x, self.mouse.y));

        self.terminal.layer(1);
        // Draw path
        if self.mouse.active {
            self.terminal.fg_color(Color::rgba8(255,255,0, 50));
            let path = engine::path_finder(self.player.x, self.player.y, 20, 20, &self.floor_map, self.map_width, self.map_height);
            for step in path.iter() {
                self.terminal.put(UI_SIZE + step.x, step.y, 219);

            }

        }
        // Mouse Display
        if self.mouse.active {
            self.terminal.fg_color(Color::rgba8(255,255,0, 100));
            self.terminal.put(UI_SIZE + self.mouse.x, self.mouse.y, 219);

        }

        self.terminal.bg_color(Color::rgb8(0,0,0));
        self.terminal.refresh(ctx);
        Ok(())
    }

    fn update(&mut self, ctx: &mut Context) -> tetra::Result {

        //engine::fov(self.player.x, self.player.y, 10, &mut self.floor_map, self.map_width, self.map_height);

        if input::is_key_pressed(ctx, Key::Left) {
            self.action_manager(Action::Move, Direction::East);
        }
        else if input::is_key_pressed(ctx, Key::Right) {
            self.action_manager(Action::Move, Direction::West);
        }
        else if input::is_key_pressed(ctx, Key::Up) {
            self.action_manager(Action::Move, Direction::North);

        }
        else if input::is_key_pressed(ctx, Key::Down) {
            self.action_manager(Action::Move, Direction::South);
        }

        if input::is_key_pressed(ctx, Key::Space) {
            self.floor_map = world::world_genration(self.map_width, self.map_height, GenerationType::Cave);
            
        }
        Ok(())

    }
    fn event(&mut self, _: &mut Context, event: Event) -> tetra::Result {
        match event {
            Event::MouseMoved{position,..} => {
                self.mouse.active = true;
                self.mouse.x = position.x as i32 / CELL_SIZE - UI_SIZE;
                self.mouse.y = position.y as i32 / CELL_SIZE;

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
