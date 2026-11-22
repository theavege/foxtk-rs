//! Snake, hosted the way [LÖVE](https://love2d.org) hosts a game.
//!
//! `load` runs once. `update(dt)` advances time. `draw` paints.
//! `keypressed` and `keyreleased` see the keyboard. FOX supplies the
//! window, a repeating timer, and the canvas. The game does not build
//! widgets.
//!
//! Arrows or WASD steer. The snake moves on a fixed step, while `dt` is
//! the time since the previous tick, as in LÖVE. Space starts again
//! after a crash. Escape quits.

use std::cell::RefCell;
use std::rc::Rc;
use std::time::Instant;

use foxtk::prelude::*;

const COLS: i32 = 32;
const ROWS: i32 = 24;
const STEP: f32 = 0.12;

mod model {
    use super::{COLS, ROWS, STEP};
    use std::collections::VecDeque;

    pub struct Snake {
        body: VecDeque<(i32, i32)>,
        dir: (i32, i32),
        next: (i32, i32),
        apple: (i32, i32),
        score: i32,
        alive: bool,
        won: bool,
        leaving: bool,
        acc: f32,
        rng: u32,
    }

    impl Snake {
        pub fn new() -> Self {
            let mut snake = Self {
                body: VecDeque::from([(4, 12), (3, 12), (2, 12)]),
                dir: (1, 0),
                next: (1, 0),
                apple: (0, 0),
                score: 0,
                alive: true,
                won: false,
                leaving: false,
                acc: 0.0,
                rng: 0xA5A5_1234,
            };
            snake.place_apple();
            snake
        }

        pub fn advance_time(&mut self, dt: f32) {
            if !self.alive {
                return;
            }
            self.acc += dt;
            let mut steps = 0;
            while self.acc >= STEP && steps < 6 {
                self.acc -= STEP;
                self.advance();
                steps += 1;
            }
            if steps == 6 {
                self.acc = 0.0;
            }
        }

        pub fn press(&mut self, key: &str) {
            if key == "escape" {
                self.leaving = true;
                return;
            }
            if !self.alive {
                if key == "space" || key == "return" {
                    self.restart();
                }
                return;
            }
            let dir = match key {
                "left" | "a" => (-1, 0),
                "right" | "d" => (1, 0),
                "up" | "w" => (0, -1),
                "down" | "s" => (0, 1),
                _ => return,
            };
            if dir.0 == -self.dir.0 && dir.1 == -self.dir.1 {
                return;
            }
            self.next = dir;
        }

        pub fn quit(&self) -> bool {
            self.leaving
        }

        pub fn alive(&self) -> bool {
            self.alive
        }

        pub fn won(&self) -> bool {
            self.won
        }

        pub fn score(&self) -> i32 {
            self.score
        }

        pub fn body(&self) -> &VecDeque<(i32, i32)> {
            &self.body
        }

        pub fn apple(&self) -> (i32, i32) {
            self.apple
        }

        fn restart(&mut self) {
            let leaving = self.leaving;
            let rng = self.rng;
            *self = Self::new();
            self.leaving = leaving;
            self.rng = rng;
        }

        fn advance(&mut self) {
            if !self.alive {
                return;
            }
            self.dir = self.next;
            let (hx, hy) = self.body[0];
            let head = (hx + self.dir.0, hy + self.dir.1);
            if head.0 < 0 || head.1 < 0 || head.0 >= COLS || head.1 >= ROWS {
                self.alive = false;
                return;
            }
            let grow = head == self.apple;
            let tail = *self.body.back().expect("snake always has a body");
            if self.body.contains(&head) && (grow || head != tail) {
                self.alive = false;
                return;
            }
            self.body.push_front(head);
            if grow {
                self.score += 1;
                self.place_apple();
            } else {
                self.body.pop_back();
            }
        }

        fn place_apple(&mut self) {
            let cells = COLS * ROWS;
            if self.body.len() as i32 >= cells {
                self.alive = false;
                self.won = true;
                return;
            }
            loop {
                let n = (self.rand() as i32).rem_euclid(cells);
                let cell = (n % COLS, n / COLS);
                if !self.body.contains(&cell) {
                    self.apple = cell;
                    return;
                }
            }
        }

        fn rand(&mut self) -> u32 {
            let mut x = self.rng;
            x ^= x << 13;
            x ^= x >> 17;
            x ^= x << 5;
            if x == 0 {
                x = 1;
            }
            self.rng = x;
            x
        }

        #[cfg(test)]
        fn at(&mut self, body: &[(i32, i32)], dir: (i32, i32), apple: (i32, i32)) {
            self.body = body.iter().copied().collect();
            self.dir = dir;
            self.next = dir;
            self.apple = apple;
            self.alive = true;
            self.won = false;
            self.score = 0;
            self.acc = 0.0;
        }
    }

    #[cfg(test)]
    mod tests {
        use super::Snake;

        #[test]
        fn time_shorter_than_a_step_does_not_move() {
            let mut snake = Snake::new();
            let head = snake.body[0];
            snake.advance_time(0.05);
            assert_eq!(snake.body[0], head);
            snake.advance_time(0.07);
            assert_eq!(snake.body[0], (head.0 + 1, head.1));
        }

        #[test]
        fn the_opposite_key_does_not_reverse() {
            let mut snake = Snake::new();
            snake.at(&[(4, 4), (3, 4), (2, 4)], (1, 0), (10, 10));
            snake.press("left");
            snake.advance_time(STEP_TIME);
            assert_eq!(snake.body[0], (5, 4));
        }

        #[test]
        fn a_turn_is_taken_on_the_next_step() {
            let mut snake = Snake::new();
            snake.at(&[(4, 4), (3, 4), (2, 4)], (1, 0), (10, 10));
            snake.press("up");
            snake.advance_time(STEP_TIME);
            assert_eq!(snake.body[0], (4, 3));
        }

        #[test]
        fn eating_grows_the_snake_and_scores() {
            let mut snake = Snake::new();
            snake.at(&[(1, 1), (0, 1)], (1, 0), (2, 1));
            snake.advance_time(STEP_TIME);
            assert_eq!(snake.score(), 1);
            assert_eq!(snake.body.len(), 3);
            assert_eq!(snake.body[0], (2, 1));
            assert_ne!(snake.apple(), (2, 1));
        }

        #[test]
        fn a_wall_ends_the_game() {
            let mut snake = Snake::new();
            snake.at(&[(0, 0), (1, 0), (2, 0)], (-1, 0), (10, 10));
            snake.advance_time(STEP_TIME);
            assert!(!snake.alive());
            assert_eq!(snake.body[0], (0, 0));
        }

        #[test]
        fn running_into_itself_ends_the_game() {
            let mut snake = Snake::new();
            snake.at(
                &[(2, 2), (1, 2), (1, 1), (2, 1), (3, 1), (3, 2)],
                (-1, 0),
                (10, 10),
            );
            snake.advance_time(STEP_TIME);
            assert!(!snake.alive());
        }

        #[test]
        fn space_after_a_crash_starts_again() {
            let mut snake = Snake::new();
            snake.at(&[(0, 0)], (-1, 0), (10, 10));
            snake.advance_time(STEP_TIME);
            assert!(!snake.alive());
            snake.press("space");
            assert!(snake.alive());
            assert_eq!(snake.score(), 0);
        }

        #[test]
        fn escape_asks_the_host_to_quit() {
            let mut snake = Snake::new();
            assert!(!snake.quit());
            snake.press("escape");
            assert!(snake.quit());
        }

        #[test]
        fn moving_onto_the_tail_is_allowed_because_the_tail_leaves() {
            let mut snake = Snake::new();
            snake.at(&[(1, 0), (0, 0), (0, 1), (1, 1)], (0, 1), (8, 8));
            snake.advance_time(STEP_TIME);
            assert!(snake.alive());
            assert_eq!(snake.body()[0], (1, 1));
            assert_eq!(snake.body().len(), 4);
        }

        const STEP_TIME: f32 = super::STEP;
    }
}

/// What LÖVE calls `love.graphics` for this game: a color, filled or
/// outlined rectangles, and a small bitmap print.
struct Graphics<'a> {
    canvas: &'a foxtk::Canvas,
    dc: &'a foxtk::DCWindow,
}

impl Graphics<'_> {
    fn set_color(&mut self, red: u8, green: u8, blue: u8) {
        self.dc
            .set_foreground(Color::from_rgb(red as u32, green as u32, blue as u32));
    }

    fn rectangle(&self, mode: &str, x: f32, y: f32, w: f32, h: f32) {
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

    fn print(&self, text: &str, x: f32, y: f32) {
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

    fn width(&self) -> f32 {
        self.canvas.width() as f32
    }

    fn height(&self) -> f32 {
        self.canvas.height() as f32
    }
}

/// The four callbacks a LÖVE game implements. Defaults do nothing, so a
/// game only writes the ones it uses.
trait Game {
    fn load(&mut self) {}
    fn update(&mut self, dt: f32);
    fn draw(&self, graphics: &mut Graphics<'_>);
    fn keypressed(&mut self, key: &str);
    fn keyreleased(&mut self, _key: &str) {}
    fn quit(&self) -> bool {
        false
    }
}

impl Game for model::Snake {
    fn update(&mut self, dt: f32) {
        self.advance_time(dt);
    }

    fn draw(&self, graphics: &mut Graphics<'_>) {
        paint(self, graphics);
    }

    fn keypressed(&mut self, key: &str) {
        self.press(key);
    }

    fn quit(&self) -> bool {
        model::Snake::quit(self)
    }
}

fn paint(snake: &model::Snake, graphics: &mut Graphics<'_>) {
    let width = graphics.width();
    let height = graphics.height();
    if width < 1.0 || height < 1.0 {
        return;
    }
    graphics.set_color(20, 24, 28);
    graphics.rectangle("fill", 0.0, 0.0, width, height);
    let size = (width / COLS as f32).min(height / ROWS as f32).max(1.0);
    let origin_x = (width - size * COLS as f32) / 2.0;
    let origin_y = (height - size * ROWS as f32) / 2.0;
    let gap = if size > 3.0 { 1.0 } else { 0.0 };
    let cell = |x: i32, y: i32| {
        (
            origin_x + x as f32 * size + gap,
            origin_y + y as f32 * size + gap,
            size - gap * 2.0,
        )
    };
    let (ax, ay, side) = cell(snake.apple().0, snake.apple().1);
    graphics.set_color(220, 72, 64);
    graphics.rectangle("fill", ax, ay, side, side);
    let last = snake.body().len().saturating_sub(1);
    for (index, (x, y)) in snake.body().iter().enumerate() {
        let (px, py, side) = cell(*x, *y);
        if index == 0 {
            graphics.set_color(186, 240, 150);
        } else if index == last {
            graphics.set_color(48, 140, 88);
        } else {
            graphics.set_color(64, 176, 104);
        }
        graphics.rectangle("fill", px, py, side, side);
    }
    graphics.set_color(232, 224, 200);
    graphics.print(
        &format!("SCORE {}", snake.score()),
        origin_x + 4.0,
        origin_y + 4.0,
    );
    if !snake.alive() {
        let banner = if snake.won() { "WIN" } else { "DEAD" };
        graphics.set_color(250, 220, 140);
        graphics.print(banner, origin_x + 4.0, origin_y + size * 2.0);
        graphics.print("SPACE", origin_x + 4.0, origin_y + size * 2.0 + 16.0);
    }
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

/// FOX host for a [`Game`]. The timer is LÖVE's frame clock: each firing
/// passes the seconds since the previous one.
fn run<G: Game + 'static>(name: &str, title: &str, width: i32, height: i32, mut game: G) -> i32 {
    let app = foxtk::App::new(name, "FoxTest");
    let window = foxtk::MainWindow::new(&app, title, width, height);
    let canvas = foxtk::Canvas::new(&window);
    canvas.set_layout(Layout::Fill);
    game.load();
    let game = Rc::new(RefCell::new(game));
    let game_for_canvas = game.clone();
    canvas.set_mouse_callback(move |canvas, code, x, _, _, _, _, _| match code {
        6 => {
            canvas.set_focus();
            let dc = foxtk::DCWindow::new(&canvas);
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

fn main() {
    run("Snake", "Snake", 640, 480, model::Snake::new());
}
