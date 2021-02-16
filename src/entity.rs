use rand::prelude::*;
use std::collections::HashMap;
use tetra::graphics::Color;

#[derive(Copy, Clone)]
pub enum ItemType {
    None,
    StoneFloor,
    //WoodFloor,
    StoneWall,
    MudWall,
    //WoodWall,
}

#[derive(Copy, Clone)]
pub enum EntityType {
    Zombie,
    Player,
}

pub enum DataValue {
    Str(&'static str),
    Int(i32),
}

pub enum DataField {
    Gold,
}

pub struct Entity {
    pub entity: EntityType,
    pub x: i32,
    pub y: i32,
    pub glyph: u8,
    pub fg_color: Color,
    pub bg_color: Color,
    pub data: HashMap<DataField, DataValue>,
}

impl Entity {
    pub fn new(x: i32, y: i32, e_type: EntityType) -> Entity {

        let mut e = Entity {
            entity: e_type,
            x: x,
            y: y,
            glyph: 0,
            fg_color: Color::rgba8(150, 150, 150, 255),
            bg_color: Color::rgba8(0, 0, 0, 0),
            data: HashMap::new(),
        };

        match e_type {
            EntityType::Player => e.create_player(),
            EntityType::Zombie => e.create_zombie(),
        }

        e
    }

    fn create_player(&mut self) {
        self.glyph = '@' as u8;
    }

    fn create_zombie(&mut self) {
        self.glyph = 'Z' as u8;
        self.fg_color = Color::rgb8(150, 100, 80);
    }
}

pub struct Tile {
    pub item: ItemType,
    pub x: i32,
    pub y: i32,
    pub glyph: u8,
    pub fg_color: Color,
    pub bg_color: Color,
    pub visible: bool,
    pub see_through: bool,
    pub crossable: bool,
    pub visited: bool,
}

impl Tile {
    pub fn new(x: i32, y: i32, style: ItemType) -> Tile {
        let mut tile = Tile {
            item: style,
            x: x,
            y: y,
            glyph: 0,
            fg_color: Color::rgb8(150, 100, 150),
            bg_color: Color::rgb8(0, 0, 0),
            visible: true,
            see_through: true,
            crossable: true,
            visited: false,
        };

        match tile.item {
            ItemType::StoneFloor => tile.stone_floor(),
            ItemType::StoneWall => tile.stone_wall(),
            ItemType::MudWall => tile.mud_wall(),
            _ => {}
        }
        tile
    }

    fn stone_floor(&mut self) {
        let mut rng = rand::thread_rng();
        let gray: u8 = rng.gen_range(60..100);
        self.fg_color = Color::rgb8(gray, gray, gray);
        self.glyph = '.' as u8;
    }

    fn mud_wall(&mut self) {
        let mut rng = rand::thread_rng();
        let rb: u8 = rng.gen_range(100..120);
        let gb: u8 = rng.gen_range(80..90);
        let bb: u8 = rng.gen_range(58..65);
        let black: u8 = rng.gen_range(5..20);
        self.glyph = '#' as u8;
        self.bg_color = Color::rgb8(rb, gb, bb);
        self.fg_color = Color::rgb8(black, black, black);
        self.crossable = false;
        self.see_through = false;
    }

    fn stone_wall(&mut self) {
        let mut rng = rand::thread_rng();
        let gray: u8 = rng.gen_range(100..150);
        let black: u8 = rng.gen_range(5..20);
        self.glyph = '#' as u8;
        self.bg_color = Color::rgb8(gray, gray, gray);
        self.fg_color = Color::rgb8(black, black, black);
        self.crossable = false;
        self.see_through = false;
    }
}
