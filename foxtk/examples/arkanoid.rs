//! Arkanoid, hosted the way [LÖVE](https://love2d.org) hosts a game.
//!
//! `load` runs once. `update(dt)` advances time. `draw` paints.
//! `keypressed` and `keyreleased` see the keyboard, and mouse motion
//! aims the paddle. FOX supplies the window, a repeating timer, and the
//! canvas. The game does not build widgets.
//!
//! Left and right, or A and D, move the paddle. The mouse does too.
//! Space launches the ball and starts again after a win or a loss.
//! Escape quits.

use std::cell::RefCell;
use std::rc::Rc;
use std::time::Instant;

use foxtk::prelude::*;

mod model {
    pub const WORLD_W: f32 = 240.0;
    pub const WORLD_H: f32 = 180.0;
    pub const PADDLE_W: f32 = 40.0;
    pub const PADDLE_H: f32 = 6.0;
    pub const PADDLE_Y: f32 = 164.0;
    pub const PADDLE_SPEED: f32 = 140.0;
    pub const BALL_R: f32 = 3.0;
    pub const BALL_SPEED: f32 = 110.0;
    const BRICK_COLS: i32 = 8;
    const BRICK_ROWS: i32 = 4;
    const BRICK_W: f32 = 26.0;
    const BRICK_H: f32 = 8.0;
    const BRICK_GAP: f32 = 2.0;
    const BRICK_TOP: f32 = 28.0;
    const MAX_STEP: f32 = 1.0 / 120.0;

    #[derive(Clone, Copy)]
    pub struct Brick {
        pub x: f32,
        pub y: f32,
        pub w: f32,
        pub h: f32,
        pub alive: bool,
        pub points: i32,
        pub row: i32,
    }

    #[derive(Clone, Copy)]
    struct Ball {
        x: f32,
        y: f32,
        vx: f32,
        vy: f32,
    }

    pub struct Arkanoid {
        paddle: f32,
        ball: Ball,
        stuck: bool,
        bricks: Vec<Brick>,
        score: i32,
        lives: i32,
        over: bool,
        won: bool,
        leaving: bool,
        left: bool,
        right: bool,
    }

    impl Arkanoid {
        pub fn new() -> Self {
            let mut game = Self {
                paddle: (WORLD_W - PADDLE_W) / 2.0,
                ball: Ball {
                    x: 0.0,
                    y: 0.0,
                    vx: 0.0,
                    vy: 0.0,
                },
                stuck: true,
                bricks: layout_bricks(),
                score: 0,
                lives: 3,
                over: false,
                won: false,
                leaving: false,
                left: false,
                right: false,
            };
            game.stick();
            game
        }

        pub fn advance_time(&mut self, dt: f32) {
            if dt <= 0.0 || self.over || self.won {
                return;
            }
            let dir = (i32::from(self.right) - i32::from(self.left)) as f32;
            self.paddle = (self.paddle + dir * PADDLE_SPEED * dt).clamp(0.0, WORLD_W - PADDLE_W);
            if self.stuck {
                self.stick();
                return;
            }
            let mut left = dt;
            while left > 0.000_001 {
                let step = left.min(MAX_STEP);
                self.integrate(step);
                left -= step;
                if self.stuck || self.over || self.won {
                    break;
                }
            }
        }

        pub fn press(&mut self, key: &str) {
            if key == "escape" {
                self.leaving = true;
                return;
            }
            match key {
                "left" | "a" => self.left = true,
                "right" | "d" => self.right = true,
                "space" | "return" if self.over || self.won => self.restart(),
                "space" | "return" if self.stuck => self.launch(),
                _ => {}
            }
        }

        pub fn release(&mut self, key: &str) {
            match key {
                "left" | "a" => self.left = false,
                "right" | "d" => self.right = false,
                _ => {}
            }
        }

        pub fn aim(&mut self, world_x: f32) {
            if self.over || self.won {
                return;
            }
            self.paddle = (world_x - PADDLE_W / 2.0).clamp(0.0, WORLD_W - PADDLE_W);
            if self.stuck {
                self.stick();
            }
        }

        pub fn quit(&self) -> bool {
            self.leaving
        }

        pub fn paddle(&self) -> f32 {
            self.paddle
        }

        pub fn ball(&self) -> (f32, f32, f32, f32) {
            (self.ball.x, self.ball.y, self.ball.vx, self.ball.vy)
        }

        pub fn bricks(&self) -> &[Brick] {
            &self.bricks
        }

        pub fn score(&self) -> i32 {
            self.score
        }

        pub fn lives(&self) -> i32 {
            self.lives
        }

        pub fn stuck(&self) -> bool {
            self.stuck
        }

        pub fn over(&self) -> bool {
            self.over
        }

        pub fn won(&self) -> bool {
            self.won
        }

        fn launch(&mut self) {
            self.stuck = false;
            self.ball.vx = BALL_SPEED * 0.35;
            self.ball.vy = -(BALL_SPEED * BALL_SPEED - self.ball.vx * self.ball.vx).sqrt();
        }

        fn restart(&mut self) {
            let leaving = self.leaving;
            *self = Self::new();
            self.leaving = leaving;
        }

        fn stick(&mut self) {
            self.ball.x = self.paddle + PADDLE_W / 2.0;
            self.ball.y = PADDLE_Y - BALL_R;
            self.ball.vx = 0.0;
            self.ball.vy = 0.0;
        }

        fn integrate(&mut self, dt: f32) {
            self.ball.x += self.ball.vx * dt;
            self.ball.y += self.ball.vy * dt;
            self.bounce_walls();
            self.bounce_paddle();
            self.bounce_bricks();
            if self.ball.y - BALL_R > WORLD_H {
                self.miss();
            }
        }

        fn bounce_walls(&mut self) {
            if self.ball.x < BALL_R {
                self.ball.x = BALL_R;
                self.ball.vx = self.ball.vx.abs();
            } else if self.ball.x > WORLD_W - BALL_R {
                self.ball.x = WORLD_W - BALL_R;
                self.ball.vx = -self.ball.vx.abs();
            }
            if self.ball.y < BALL_R {
                self.ball.y = BALL_R;
                self.ball.vy = self.ball.vy.abs();
            }
        }

        fn bounce_paddle(&mut self) {
            if self.ball.vy <= 0.0
                || !overlaps(
                    self.ball.x,
                    self.ball.y,
                    self.paddle,
                    PADDLE_Y,
                    PADDLE_W,
                    PADDLE_H,
                )
            {
                return;
            }
            let center = self.paddle + PADDLE_W / 2.0;
            let offset = ((self.ball.x - center) / (PADDLE_W / 2.0)).clamp(-1.0, 1.0);
            self.ball.vx = offset * BALL_SPEED * 0.85;
            let rise = BALL_SPEED * BALL_SPEED - self.ball.vx * self.ball.vx;
            self.ball.vy = -rise.max(0.0).sqrt();
            self.ball.y = PADDLE_Y - BALL_R;
        }

        fn bounce_bricks(&mut self) {
            let Some(index) = self.bricks.iter().position(|brick| {
                brick.alive
                    && overlaps(self.ball.x, self.ball.y, brick.x, brick.y, brick.w, brick.h)
            }) else {
                return;
            };
            let brick = self.bricks[index];
            reflect(&mut self.ball, brick);
            self.bricks[index].alive = false;
            self.score += brick.points;
            if self.bricks.iter().all(|brick| !brick.alive) {
                self.won = true;
            }
        }

        fn miss(&mut self) {
            self.lives -= 1;
            if self.lives <= 0 {
                self.over = true;
                self.stuck = false;
            } else {
                self.stuck = true;
                self.stick();
            }
        }

        #[cfg(test)]
        fn with_ball(&mut self, x: f32, y: f32, vx: f32, vy: f32) {
            self.stuck = false;
            self.over = false;
            self.won = false;
            self.ball = Ball { x, y, vx, vy };
        }

        #[cfg(test)]
        fn only_brick(&mut self, index: usize) {
            for (i, brick) in self.bricks.iter_mut().enumerate() {
                brick.alive = i == index;
            }
        }
    }

    fn layout_bricks() -> Vec<Brick> {
        let span = BRICK_COLS as f32 * BRICK_W + (BRICK_COLS - 1) as f32 * BRICK_GAP;
        let origin = (WORLD_W - span) / 2.0;
        let mut bricks = Vec::with_capacity((BRICK_COLS * BRICK_ROWS) as usize);
        for row in 0..BRICK_ROWS {
            for col in 0..BRICK_COLS {
                bricks.push(Brick {
                    x: origin + col as f32 * (BRICK_W + BRICK_GAP),
                    y: BRICK_TOP + row as f32 * (BRICK_H + BRICK_GAP),
                    w: BRICK_W,
                    h: BRICK_H,
                    alive: true,
                    points: (BRICK_ROWS - row) * 10,
                    row,
                });
            }
        }
        bricks
    }

    fn overlaps(cx: f32, cy: f32, x: f32, y: f32, w: f32, h: f32) -> bool {
        let nearest_x = cx.clamp(x, x + w);
        let nearest_y = cy.clamp(y, y + h);
        let dx = cx - nearest_x;
        let dy = cy - nearest_y;
        dx * dx + dy * dy <= BALL_R * BALL_R
    }

    fn reflect(ball: &mut Ball, brick: Brick) {
        let dx = ball.x - (brick.x + brick.w / 2.0);
        let dy = ball.y - (brick.y + brick.h / 2.0);
        if (brick.w * dy).abs() > (brick.h * dx).abs() {
            if dy < 0.0 {
                ball.vy = -ball.vy.abs();
                ball.y = brick.y - BALL_R;
            } else {
                ball.vy = ball.vy.abs();
                ball.y = brick.y + brick.h + BALL_R;
            }
        } else if dx < 0.0 {
            ball.vx = -ball.vx.abs();
            ball.x = brick.x - BALL_R;
        } else {
            ball.vx = ball.vx.abs();
            ball.x = brick.x + brick.w + BALL_R;
        }
    }

    #[cfg(test)]
    mod tests {
        use super::{Arkanoid, BALL_R, PADDLE_W, PADDLE_Y, WORLD_H, WORLD_W};

        #[test]
        fn the_paddle_moves_while_a_key_is_held_and_stops_at_the_wall() {
            let mut game = Arkanoid::new();
            let start = game.paddle();
            game.press("right");
            game.advance_time(0.1);
            assert!(game.paddle() > start);
            game.release("right");
            let stopped = game.paddle();
            game.advance_time(0.1);
            assert_eq!(game.paddle(), stopped);
            game.press("left");
            game.advance_time(5.0);
            assert_eq!(game.paddle(), 0.0);
        }

        #[test]
        fn a_stuck_ball_follows_the_paddle() {
            let mut game = Arkanoid::new();
            game.press("right");
            game.advance_time(0.1);
            let (x, y, _, _) = game.ball();
            assert!((x - (game.paddle() + PADDLE_W / 2.0)).abs() < 0.01);
            assert!((y - (PADDLE_Y - BALL_R)).abs() < 0.01);
        }

        #[test]
        fn space_launches_the_ball_upward() {
            let mut game = Arkanoid::new();
            game.press("space");
            assert!(!game.stuck());
            let (_, _, _, vy) = game.ball();
            assert!(vy < 0.0);
        }

        #[test]
        fn the_ball_bounces_off_the_side_walls_and_the_ceiling() {
            let mut game = Arkanoid::new();
            game.with_ball(BALL_R + 0.2, 80.0, -90.0, 0.0);
            game.advance_time(0.02);
            let (x, _, vx, _) = game.ball();
            assert!(vx > 0.0);
            assert!(x >= BALL_R - 0.01);

            game.with_ball(WORLD_W - BALL_R - 0.2, 80.0, 90.0, 0.0);
            game.advance_time(0.02);
            let (x, _, vx, _) = game.ball();
            assert!(vx < 0.0);
            assert!(x <= WORLD_W - BALL_R + 0.01);

            game.with_ball(40.0, BALL_R + 0.2, 0.0, -90.0);
            game.advance_time(0.02);
            let (_, y, _, vy) = game.ball();
            assert!(vy > 0.0);
            assert!(y >= BALL_R - 0.01);
        }

        #[test]
        fn the_paddle_sends_a_left_hit_back_up_and_left() {
            let mut game = Arkanoid::new();
            let x = game.paddle() + 4.0;
            game.with_ball(x, PADDLE_Y - BALL_R - 0.3, 0.0, 140.0);
            game.advance_time(0.02);
            let (_, _, vx, vy) = game.ball();
            assert!(vy < 0.0);
            assert!(vx < 0.0);
        }

        #[test]
        fn a_brick_breaks_scores_and_the_last_one_wins() {
            let mut game = Arkanoid::new();
            let brick = game.bricks()[0];
            game.with_ball(brick.x + brick.w / 2.0, brick.y - BALL_R - 0.4, 0.0, 100.0);
            game.advance_time(0.02);
            assert!(!game.bricks()[0].alive);
            assert_eq!(game.score(), brick.points);
            let (_, _, _, vy) = game.ball();
            assert!(vy < 0.0);

            game.only_brick(3);
            let brick = game.bricks()[3];
            game.with_ball(
                brick.x + brick.w / 2.0,
                brick.y + brick.h + BALL_R + 0.4,
                0.0,
                -100.0,
            );
            game.advance_time(0.02);
            assert!(game.won());
            assert!(!game.bricks()[3].alive);
        }

        #[test]
        fn missing_the_paddle_costs_a_life_and_the_last_life_ends_it() {
            let mut game = Arkanoid::new();
            game.with_ball(20.0, WORLD_H - 1.0, 0.0, 80.0);
            game.advance_time(0.1);
            assert_eq!(game.lives(), 2);
            assert!(game.stuck());
            game.with_ball(20.0, WORLD_H - 1.0, 0.0, 80.0);
            game.advance_time(0.1);
            game.with_ball(20.0, WORLD_H - 1.0, 0.0, 80.0);
            game.advance_time(0.1);
            assert!(game.over());
            assert_eq!(game.lives(), 0);
            game.press("space");
            assert!(!game.over());
            assert!(game.stuck());
            assert_eq!(game.lives(), 3);
            assert_eq!(game.score(), 0);
        }

        #[test]
        fn the_mouse_aims_the_paddle() {
            let mut game = Arkanoid::new();
            game.aim(WORLD_W - 1.0);
            assert!((game.paddle() - (WORLD_W - PADDLE_W)).abs() < 0.01);
        }

        #[test]
        fn escape_asks_the_host_to_quit() {
            let mut game = Arkanoid::new();
            assert!(!game.quit());
            game.press("escape");
            assert!(game.quit());
        }
    }
}

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

    fn circle(&self, x: f32, y: f32, radius: f32) {
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

trait Game {
    fn load(&mut self) {}
    fn update(&mut self, dt: f32);
    fn draw(&self, graphics: &mut Graphics<'_>);
    fn keypressed(&mut self, key: &str);
    fn keyreleased(&mut self, _key: &str) {}
    fn mousemoved(&mut self, _x: f32) {}
    fn quit(&self) -> bool {
        false
    }
}

impl Game for model::Arkanoid {
    fn update(&mut self, dt: f32) {
        self.advance_time(dt);
    }

    fn draw(&self, graphics: &mut Graphics<'_>) {
        paint(self, graphics);
    }

    fn keypressed(&mut self, key: &str) {
        self.press(key);
    }

    fn keyreleased(&mut self, key: &str) {
        self.release(key);
    }

    fn mousemoved(&mut self, x: f32) {
        self.aim(x);
    }

    fn quit(&self) -> bool {
        model::Arkanoid::quit(self)
    }
}

fn frame(width: f32, height: f32) -> (f32, f32, f32) {
    if width < 1.0 || height < 1.0 {
        return (0.0, 0.0, 1.0);
    }
    let scale = (width / model::WORLD_W).min(height / model::WORLD_H);
    let origin_x = (width - model::WORLD_W * scale) / 2.0;
    let origin_y = (height - model::WORLD_H * scale) / 2.0;
    (origin_x, origin_y, scale)
}

fn paint(game: &model::Arkanoid, graphics: &mut Graphics<'_>) {
    let width = graphics.width();
    let height = graphics.height();
    if width < 1.0 || height < 1.0 {
        return;
    }
    graphics.set_color(12, 16, 28);
    graphics.rectangle("fill", 0.0, 0.0, width, height);
    let (origin_x, origin_y, scale) = frame(width, height);
    let place = |x: f32, y: f32, w: f32, h: f32| {
        (
            origin_x + x * scale,
            origin_y + y * scale,
            (w * scale).max(1.0),
            (h * scale).max(1.0),
        )
    };
    let (field_x, field_y, field_w, field_h) = place(0.0, 0.0, model::WORLD_W, model::WORLD_H);
    graphics.set_color(18, 24, 40);
    graphics.rectangle("fill", field_x, field_y, field_w, field_h);
    for brick in game.bricks().iter().filter(|brick| brick.alive) {
        let (red, green, blue) = match brick.row {
            0 => (220, 72, 64),
            1 => (220, 160, 48),
            2 => (72, 176, 96),
            _ => (64, 140, 210),
        };
        graphics.set_color(red, green, blue);
        let (x, y, w, h) = place(brick.x, brick.y, brick.w, brick.h);
        graphics.rectangle("fill", x, y, w, h);
    }
    graphics.set_color(232, 224, 200);
    let (x, y, w, h) = place(
        game.paddle(),
        model::PADDLE_Y,
        model::PADDLE_W,
        model::PADDLE_H,
    );
    graphics.rectangle("fill", x, y, w, h);
    let (ball_x, ball_y, _, _) = game.ball();
    graphics.set_color(248, 248, 252);
    graphics.circle(
        origin_x + ball_x * scale,
        origin_y + ball_y * scale,
        (model::BALL_R * scale).max(2.0),
    );
    graphics.set_color(232, 224, 200);
    graphics.print(
        &format!("SCORE {}", game.score()),
        origin_x + 4.0,
        origin_y + 4.0,
    );
    graphics.print(
        &format!("LIVES {}", game.lives()),
        origin_x + model::WORLD_W * scale - 72.0,
        origin_y + 4.0,
    );
    if game.stuck() && !game.over() {
        graphics.print(
            "SPACE",
            origin_x + 4.0,
            origin_y + model::WORLD_H * scale - 18.0,
        );
    }
    if game.over() || game.won() {
        graphics.set_color(250, 220, 140);
        let banner = if game.won() { "WIN" } else { "DEAD" };
        graphics.print(banner, origin_x + 4.0, origin_y + 28.0);
        graphics.print("SPACE", origin_x + 4.0, origin_y + 44.0);
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

fn run<G: Game + 'static>(name: &str, title: &str, width: i32, height: i32, mut game: G) -> i32 {
    let app = foxtk::App::new(name, "FoxTest");
    let window = foxtk::MainWindow::new(&app, title, width, height);
    let canvas = foxtk::Canvas::new(&window);
    canvas.set_layout(Layout::Fill);
    game.load();
    let game = Rc::new(RefCell::new(game));
    let game_for_canvas = game.clone();
    canvas.set_mouse_callback(move |canvas, code, x, _, _, _, _, _| match code {
        3 => {
            let width = canvas.width() as f32;
            let height = canvas.height() as f32;
            let (origin_x, _, scale) = frame(width, height);
            game_for_canvas
                .borrow_mut()
                .mousemoved((x as f32 - origin_x) / scale);
            false
        }
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
    run("Arkanoid", "Arkanoid", 640, 480, model::Arkanoid::new());
}
