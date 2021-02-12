use tetra::{Context, ContextBuilder, State};
use tetra::graphics::Color;
use tetra::input::{self, Key};

mod entity;
use entity::{Entity, Tile, ItemType};

mod world;
use world::*;

mod grl;
use grl::Terminal;


const VIEW_WIDTH: i32 = 60;
const VIEW_HEIGHT: i32 = 40;
const UI_SIZE: i32 = 20;
const HEIGHT: i32 = VIEW_HEIGHT;
const WIDTH: i32 = VIEW_WIDTH + UI_SIZE;
const CELL_SIZE: i32 = 16;


struct GameState {
    terminal: Terminal,
    floor_map: Vec<Tile>,
    player: Entity,
    
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        Ok(GameState {
            terminal: Terminal::new(ctx, WIDTH, HEIGHT, CELL_SIZE, CELL_SIZE),
            player: Entity::new(10, 10, 64),
            floor_map: world::world_genration(VIEW_WIDTH, VIEW_HEIGHT, GenerationType::Cave),

        })
    }
}

impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        // Cornflower blue, as is tradition
        self.terminal.clear(ctx);

        // Map display

        for tile in self.floor_map.iter() {
            self.terminal.fg_color(tile.fg_color);
            self.terminal.bg_color(tile.bg_color);
            self.terminal.put(UI_SIZE + tile.x, tile.y, tile.glyph);
        }

        // Player display
        let player_index = (self.player.y * VIEW_WIDTH + self.player.x) as usize; 
        self.terminal.bg_color(self.floor_map[player_index].bg_color);
        self.terminal.fg_color(self.player.fg_color);
        self.terminal.put(UI_SIZE + self.player.x, self.player.y, self.player.glyph);

        self.terminal.bg_color(Color::rgb8(0,0,0));
        self.terminal.print(0,0, format!("Position {} - {}", self.player.x, self.player.y));

        self.terminal.bg_color(Color::rgb8(0,0,0));
        self.terminal.refresh(ctx);
        Ok(())
    }

    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        if input::is_key_pressed(ctx, Key::Left) {
            self.player.x -= 1;
        }
        else if input::is_key_pressed(ctx, Key::Right) {
            self.player.x += 1;
        }
        else if input::is_key_pressed(ctx, Key::Up) {
            self.player.y -= 1;

        }
        else if input::is_key_pressed(ctx, Key::Down) {
            self.player.y += 1
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
