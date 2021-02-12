use tetra::graphics::{self, Color, DrawParams, Rectangle, Texture};
use tetra::Context;
use tetra::math::Vec2;

struct Cell {
    x: i32,
    y: i32,
    glyph: u8,
    fg_color: Color,
    bg_color: Color,
}

pub struct Terminal {
  w: i32,
  h: i32,
  cell_w: i32,
  cell_h: i32,
  fg_color: Color,
  bg_color: Color,
  tileset: Texture,
  console: Vec<Cell>
}

impl Terminal {
    pub fn new(ctx: &mut Context, w: i32, h: i32, cell_w: i32, cell_h: i32) -> Terminal {
        let ts: Texture;
        match cell_w {
            24 => ts = Texture::new(ctx, "./assets/24x24.png").unwrap(),
            16 => ts = Texture::new(ctx, "./assets/16x16.png").unwrap(),
            _ => ts = Texture::new(ctx, "./assets/24x24.png").unwrap()

        }
        let bg = Color::rgb8(0, 0, 0);
        let fg = Color::rgb8(255, 255, 255);
        let mut cons = Vec::new();
        for i in 0..(w*h) as usize {
            cons.push(
                Cell{
                    x: (i as i32) % w,
                    y: (i as i32) / w,
                    glyph: 0,
                    fg_color: fg,
                    bg_color: bg,
                });
        }
        Terminal {
            w: w,
            h: h,
            cell_w: cell_w,
            cell_h: cell_h,
            fg_color: Color::rgb8(200, 200, 200),
            bg_color: Color::rgb8(0, 0, 0),
            tileset: ts,
            console: cons,
        }
    }
    pub fn clear(&mut self, ctx: &mut Context) {
        graphics::clear(ctx, self.bg_color);
        for cell in self.console.iter_mut() {
            cell.glyph = 0;
            cell.bg_color = self.bg_color;
            cell.fg_color = self.fg_color;
        }
    }

    pub fn fg_color(&mut self, c: Color) {
        self.fg_color = c;
    }

    pub fn bg_color(&mut self, c: Color) {
        self.bg_color = c;
    }

    pub fn pick(&self, x: i32, y: i32) -> u8 {
        let index = y * self.w + x;
        self.console[index as usize].glyph
    }
    pub fn pick_bg_color(&self, x: i32, y: i32) -> Color {
        let index = y * self.w + x;
        self.console[index as usize].bg_color
    }
    pub fn pick_fg_color(&self, x: i32, y: i32) -> Color {
        let index = y * self.w + x;
        self.console[index as usize].fg_color
    }

    pub fn put(&mut self, x: i32, y: i32, glyph: u8) {
        let index = y * self.w + x;
        self.console[index as usize].glyph =  glyph;
        self.console[index as usize].fg_color = self.fg_color;
        self.console[index as usize].bg_color = self.bg_color;
    }

    pub fn print(&mut self, x: i32, y: i32, string: String) {
        for (i,letter) in string.chars().enumerate() {
            self.put(x + i as i32, y, letter as u8);

        }
    }

    pub fn refresh(&mut self, ctx: &mut Context) {
        for cell in self.console.iter_mut() {
            let tx = cell.glyph as i32 % 16;
            let ty = cell.glyph as i32 / 16;
            let bx = 219 % 16;
            let by = 219 / 16;
            self.tileset.draw_region(
                ctx,
                Rectangle::new((self.cell_w * bx) as f32, (self.cell_h * by) as f32, self.cell_w as f32, self.cell_h as f32),
                DrawParams::new()
                    .position(Vec2::new((cell.x * self.cell_w) as f32, (cell.y * self.cell_h) as f32))
                    .color(cell.bg_color),

            );
            self.tileset.draw_region(
                ctx,
                Rectangle::new((self.cell_w * tx) as f32, (self.cell_h * ty) as f32, self.cell_w as f32, self.cell_h as f32),
                DrawParams::new()
                    .position(Vec2::new((cell.x * self.cell_w) as f32, (cell.y * self.cell_h) as f32))
                    .color(cell.fg_color),

            );
            
        }
    }
}
