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

fn main() {
    model::Snake::new().run(Args {
        name: "Snake",
        title: "Snake",
        width: 640,
        height: 480,
    });
}
