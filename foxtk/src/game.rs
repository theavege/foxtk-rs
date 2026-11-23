//! A LÖVE-style game loop on a FOX canvas.
//!
//! A game implements [`Game`]. [`Game::run`] opens one window and calls
//! `load`, then `update(dt)`, `draw`, `keypressed`, `keyreleased`, and the
//! mouse callbacks. `dt` is the seconds since the previous timer firing.

use std::cell::RefCell;
use std::rc::Rc;
use std::time::Instant;

use crate::prelude::{Color, DrawableExt, Layout, ObjectExt, WindowExt};

/// Arguments for [`Game::run`]. `name` is the FOX application name.
pub struct Args<'a> {
    pub name: &'a str,
    pub title: &'a str,
    pub width: i32,
    pub height: i32,
}

/// Drawing commands for one paint. Coordinates are canvas pixels.
pub struct Graphics<'a> {
    canvas: &'a crate::Canvas,
    dc: &'a crate::DCWindow,
}

impl Graphics<'_> {
    pub fn set_color(&mut self, red: u8, green: u8, blue: u8) {
        self.dc
            .set_foreground(Color::from_rgb(red as u32, green as u32, blue as u32));
    }

    /// `mode` is `"fill"` or `"line"`.
    pub fn rectangle(&self, mode: &str, x: f32, y: f32, w: f32, h: f32) {
        let (x, y, w, h) = (x as i32, y as i32, w.max(1.0) as i32, h.max(1.0) as i32);
        if mode == "line" {
            self.dc.draw_line(x, y, x + w, y);
            self.dc.draw_line(x + w, y, x + w, y + h);
            self.dc.draw_line(x + w, y + h, x, y + h);
            self.dc.draw_line(x, y + h, x, y);
        } else {
            self.dc.fill_rectangle(x, y, w, h);
        }
    }

    pub fn circle(&self, x: f32, y: f32, radius: f32) {
        let diameter = (radius * 2.0).max(2.0) as i32;
        self.dc.fill_arc(
            (x - radius) as i32,
            (y - radius) as i32,
            diameter,
            diameter,
            0,
            360 * 64,
        );
    }

    pub fn print(&self, text: &str, x: f32, y: f32) {
        let scale = 2;
        let mut cursor = x as i32;
        let top = y as i32;
        for ch in text.chars() {
            let rows = glyph(ch);
            for (row, bits) in rows.iter().enumerate() {
                for col in 0..3 {
                    if bits & (1 << (2 - col)) == 0 {
                        continue;
                    }
                    self.dc.fill_rectangle(
                        cursor + col * scale,
                        top + row as i32 * scale,
                        scale,
                        scale,
                    );
                }
            }
            cursor += 4 * scale;
        }
    }

    pub fn width(&self) -> f32 {
        self.canvas.width() as f32
    }

    pub fn height(&self) -> f32 {
        self.canvas.height() as f32
    }
}

/// Callbacks a LÖVE game implements. Defaults do nothing, so a game only
/// writes the ones it uses.
pub trait Game: Sized + 'static {
    fn load(&mut self) {}
    fn update(&mut self, dt: f32);
    fn draw(&self, graphics: &mut Graphics<'_>);
    fn keypressed(&mut self, key: &str);
    fn keyreleased(&mut self, _key: &str) {}
    /// `button` is 1 for the left button and 2 for the right button.
    /// `x` and `y` are canvas pixels.
    fn mousepressed(&mut self, _button: i32, _x: f32, _y: f32, _width: f32, _height: f32) {}
    fn mousemoved(&mut self, _x: f32, _y: f32, _width: f32, _height: f32) {}
    fn quit(&self) -> bool {
        false
    }

    /// Open the window and run until it closes.
    fn run(self, args: Args<'_>) -> i32 {
        run_game(self, args)
    }
}

fn run_game<G: Game>(mut game: G, args: Args<'_>) -> i32 {
    let app = crate::App::new(args.name, "FoxTest");
    let window = crate::MainWindow::new(&app, args.title, args.width, args.height);
    let canvas = crate::Canvas::new(&window);
    canvas.set_layout(Layout::Fill);
    game.load();
    let game = Rc::new(RefCell::new(game));
    let game_for_canvas = game.clone();
    canvas.set_mouse_callback(move |canvas, code, x, y, _, _, _, _| {
        let width = canvas.width() as f32;
        let height = canvas.height() as f32;
        let px = x as f32;
        let py = y as f32;
        match code {
            1 => {
                game_for_canvas
                    .borrow_mut()
                    .mousepressed(1, px, py, width, height);
                canvas.refresh();
                true
            }
            3 => {
                game_for_canvas
                    .borrow_mut()
                    .mousemoved(px, py, width, height);
                false
            }
            4 => {
                game_for_canvas
                    .borrow_mut()
                    .mousepressed(2, px, py, width, height);
                canvas.refresh();
                true
            }
            6 => {
                canvas.set_focus();
                let dc = crate::DCWindow::new(&canvas);
                {
                    let mut graphics = Graphics {
                        canvas: &canvas,
                        dc: &dc,
                    };
                    game_for_canvas.borrow().draw(&mut graphics);
                }
                dc.finish();
                true
            }
            7 => {
                game_for_canvas.borrow_mut().keypressed(key_name(x));
                canvas.refresh();
                true
            }
            8 => {
                game_for_canvas.borrow_mut().keyreleased(key_name(x));
                true
            }
            _ => false,
        }
    });
    let canvas_for_tick = canvas.alias();
    let game_for_tick = game.clone();
    let mut previous = Instant::now();
    let _timer = app.add_timeout(16, move |_| {
        let now = Instant::now();
        let dt = (now - previous).as_secs_f32().clamp(0.0, 0.1);
        previous = now;
        {
            let mut game = game_for_tick.borrow_mut();
            game.update(dt);
        }
        if game_for_tick.borrow().quit() {
            canvas_for_tick.close_shell();
        }
        canvas_for_tick.refresh();
        false
    });
    window.show();
    canvas.set_focus();
    app.run()
}

fn glyph(ch: char) -> [u8; 5] {
    match ch.to_ascii_uppercase() {
        'A' => [0b111, 0b101, 0b111, 0b101, 0b101],
        'B' => [0b110, 0b101, 0b110, 0b101, 0b110],
        'C' => [0b111, 0b100, 0b100, 0b100, 0b111],
        'D' => [0b110, 0b101, 0b101, 0b101, 0b110],
        'E' => [0b111, 0b100, 0b110, 0b100, 0b111],
        'F' => [0b111, 0b100, 0b110, 0b100, 0b100],
        'G' => [0b111, 0b100, 0b101, 0b101, 0b111],
        'H' => [0b101, 0b101, 0b111, 0b101, 0b101],
        'I' => [0b111, 0b010, 0b010, 0b010, 0b111],
        'J' => [0b001, 0b001, 0b001, 0b101, 0b111],
        'K' => [0b101, 0b101, 0b110, 0b101, 0b101],
        'L' => [0b100, 0b100, 0b100, 0b100, 0b111],
        'M' => [0b101, 0b111, 0b111, 0b101, 0b101],
        'N' => [0b111, 0b101, 0b101, 0b101, 0b101],
        'O' => [0b111, 0b101, 0b101, 0b101, 0b111],
        'P' => [0b111, 0b101, 0b111, 0b100, 0b100],
        'Q' => [0b111, 0b101, 0b101, 0b111, 0b001],
        'R' => [0b111, 0b101, 0b111, 0b110, 0b101],
        'S' => [0b111, 0b100, 0b111, 0b001, 0b111],
        'T' => [0b111, 0b010, 0b010, 0b010, 0b010],
        'U' => [0b101, 0b101, 0b101, 0b101, 0b111],
        'V' => [0b101, 0b101, 0b101, 0b101, 0b010],
        'W' => [0b101, 0b101, 0b111, 0b111, 0b101],
        'X' => [0b101, 0b101, 0b010, 0b101, 0b101],
        'Y' => [0b101, 0b101, 0b111, 0b010, 0b010],
        'Z' => [0b111, 0b001, 0b010, 0b100, 0b111],
        '0' => [0b111, 0b101, 0b101, 0b101, 0b111],
        '1' => [0b010, 0b110, 0b010, 0b010, 0b111],
        '2' => [0b111, 0b001, 0b111, 0b100, 0b111],
        '3' => [0b111, 0b001, 0b111, 0b001, 0b111],
        '4' => [0b101, 0b101, 0b111, 0b001, 0b001],
        '5' => [0b111, 0b100, 0b111, 0b001, 0b111],
        '6' => [0b111, 0b100, 0b111, 0b101, 0b111],
        '7' => [0b111, 0b001, 0b010, 0b010, 0b010],
        '8' => [0b111, 0b101, 0b111, 0b101, 0b111],
        '9' => [0b111, 0b101, 0b111, 0b001, 0b111],
        _ => [0, 0, 0, 0, 0],
    }
}

fn key_name(code: i32) -> &'static str {
    match code {
        0xFF51 | 0xFF96 => "left",
        0xFF52 | 0xFF97 => "up",
        0xFF53 | 0xFF98 => "right",
        0xFF54 | 0xFF99 => "down",
        0x0020 | 0xFF80 => "space",
        0xFF1B => "escape",
        0xFF0D | 0xFF8D => "return",
        0x0061 | 0x0041 => "a",
        0x0064 | 0x0044 => "d",
        0x0073 | 0x0053 => "s",
        0x0077 | 0x0057 => "w",
        _ => "",
    }
}
