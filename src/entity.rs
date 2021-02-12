use tetra::graphics::Color;
use rand::prelude::*;

#[derive(Copy, Clone)]
pub enum ItemType {
    None,
    StoneFloor,
    //WoodFloor,
    StoneWall,
    MudWall,
    //WoodWall,
}

pub struct Entity {
    pub x: i32,
    pub y: i32,
    pub glyph: u8,
    pub fg_color: Color,
    pub bg_color: Color,
}

impl Entity {
    pub fn new(x: i32, y: i32, glyph: u8) -> Entity {
        Entity {
            x: x,
            y: y,
            glyph: glyph,
            fg_color: Color::rgba8(150, 150, 150, 255),
            bg_color: Color::rgba8(0, 0, 0, 0),
        }
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
    pub crossable: bool,
}

impl Tile {
    pub fn new(x: i32, y: i32, style: ItemType) -> Tile {
        let mut tile = Tile {
            item: style,
            x: x,
            y: y,
            glyph: 0,
            fg_color: Color::rgb8(150,100,150),
            bg_color: Color::rgb8(0,0,0),
            visible: true,
            crossable: true,
        };
        
        match tile.item {
            ItemType::StoneFloor => tile.stone_floor(),
            ItemType::StoneWall => tile.stone_wall(),
            ItemType::MudWall => tile.mud_wall(),
            _ => {},
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

    }

    fn stone_wall(&mut self) {
        let mut rng = rand::thread_rng();
        let gray: u8 = rng.gen_range(100..150);
        let black: u8 = rng.gen_range(5..20);
        self.glyph = '#' as u8;
        self.bg_color = Color::rgb8(gray, gray, gray);
        self.fg_color = Color::rgb8(black, black, black);
        self.crossable = false;
    }
}