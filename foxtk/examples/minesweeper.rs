//! Minesweeper, hosted the way [LÖVE](https://love2d.org) hosts a game.
//!
//! `load` runs once. `update(dt)` advances time. `draw` paints.
//! `mousepressed` opens a cell or plants a flag, and `keypressed` sees
//! the keyboard. FOX supplies the window, a repeating timer, and the
//! canvas. The game does not build widgets.
//!
//! The left button opens a cell. The first open is never a mine, and a
//! zero opens its neighbors. The right button flags a cell. Space starts
//! again after a win or a loss. Escape quits.

use foxtk::prelude::*;

mod model {
    pub const COLS: i32 = 9;
    pub const ROWS: i32 = 9;
    const MINES: usize = 10;

    #[derive(Clone, Copy)]
    pub struct Cell {
        pub mine: bool,
        pub open: bool,
        pub flag: bool,
        pub adjacent: u8,
    }

    pub struct Minesweeper {
        cols: i32,
        rows: i32,
        cells: Vec<Cell>,
        alive: bool,
        won: bool,
        leaving: bool,
        placed: bool,
        exploded: Option<usize>,
        rng: u32,
    }

    impl Minesweeper {
        pub fn new() -> Self {
            Self::blank(COLS, ROWS)
        }

        pub fn open_at(&mut self, col: i32, row: i32) {
            if !self.alive || self.won {
                return;
            }
            let Some(index) = self.index(col, row) else {
                return;
            };
            if self.cells[index].open || self.cells[index].flag {
                return;
            }
            if !self.placed {
                self.place(index);
            }
            if self.cells[index].mine {
                self.alive = false;
                self.exploded = Some(index);
                return;
            }
            self.reveal(index);
        }

        pub fn flag_at(&mut self, col: i32, row: i32) {
            if !self.alive || self.won {
                return;
            }
            let Some(index) = self.index(col, row) else {
                return;
            };
            if !self.cells[index].open {
                self.cells[index].flag = !self.cells[index].flag;
            }
        }

        pub fn press(&mut self, key: &str) {
            if key == "escape" {
                self.leaving = true;
            } else if matches!(key, "space" | "return") && (!self.alive || self.won) {
                let leaving = self.leaving;
                *self = Self::blank(self.cols, self.rows);
                self.leaving = leaving;
            }
        }

        pub fn quit(&self) -> bool {
            self.leaving
        }

        pub fn cols(&self) -> i32 {
            self.cols
        }

        pub fn rows(&self) -> i32 {
            self.rows
        }

        pub fn alive(&self) -> bool {
            self.alive
        }

        pub fn won(&self) -> bool {
            self.won
        }

        pub fn mine_count(&self) -> i32 {
            self.cells.iter().filter(|cell| cell.mine).count() as i32
        }

        pub fn flag_count(&self) -> i32 {
            self.cells.iter().filter(|cell| cell.flag).count() as i32
        }

        pub fn cell(&self, col: i32, row: i32) -> Option<Cell> {
            self.index(col, row).map(|index| self.cells[index])
        }

        pub fn exploded(&self) -> Option<(i32, i32)> {
            self.exploded.map(|index| self.coords(index))
        }

        pub fn frame(&self, width: f32, height: f32) -> (f32, f32, f32) {
            if width < 1.0 || height < 1.0 || self.cols < 1 || self.rows < 1 {
                return (0.0, 0.0, 1.0);
            }
            let size = (width / self.cols as f32)
                .min(height / self.rows as f32)
                .max(1.0);
            let origin_x = (width - size * self.cols as f32) / 2.0;
            let origin_y = (height - size * self.rows as f32) / 2.0;
            (origin_x, origin_y, size)
        }

        pub fn cell_at(&self, x: f32, y: f32, width: f32, height: f32) -> Option<(i32, i32)> {
            let (origin_x, origin_y, size) = self.frame(width, height);
            let col_f = (x - origin_x) / size;
            let row_f = (y - origin_y) / size;
            if col_f < 0.0 || row_f < 0.0 {
                return None;
            }
            let col = col_f as i32;
            let row = row_f as i32;
            self.index(col, row).map(|_| (col, row))
        }

        fn blank(cols: i32, rows: i32) -> Self {
            let empty = Cell {
                mine: false,
                open: false,
                flag: false,
                adjacent: 0,
            };
            Self {
                cols,
                rows,
                cells: vec![empty; (cols * rows) as usize],
                alive: true,
                won: false,
                leaving: false,
                placed: false,
                exploded: None,
                rng: 0x4d5a_6e65,
            }
        }

        fn place(&mut self, safe: usize) {
            self.placed = true;
            let mut spots: Vec<usize> = (0..self.cells.len())
                .filter(|&index| index != safe)
                .collect();
            for index in (1..spots.len()).rev() {
                let swap = self.next_u32() as usize % (index + 1);
                spots.swap(index, swap);
            }
            let mines = MINES.min(spots.len());
            for index in spots.into_iter().take(mines) {
                self.cells[index].mine = true;
            }
            self.recount();
        }

        fn reveal(&mut self, start: usize) {
            let mut stack = vec![start];
            while let Some(index) = stack.pop() {
                if self.cells[index].open || self.cells[index].flag || self.cells[index].mine {
                    continue;
                }
                self.cells[index].open = true;
                if self.cells[index].adjacent == 0 {
                    stack.extend(self.neighbors(index));
                }
            }
            let covered = self.cells.iter().any(|cell| !cell.mine && !cell.open);
            if !covered {
                self.won = true;
            }
        }

        fn recount(&mut self) {
            for index in 0..self.cells.len() {
                if self.cells[index].mine {
                    self.cells[index].adjacent = 0;
                    continue;
                }
                let adjacent = self
                    .neighbors(index)
                    .into_iter()
                    .filter(|&next| self.cells[next].mine)
                    .count() as u8;
                self.cells[index].adjacent = adjacent;
            }
        }

        fn index(&self, col: i32, row: i32) -> Option<usize> {
            if col < 0 || row < 0 || col >= self.cols || row >= self.rows {
                None
            } else {
                Some((row * self.cols + col) as usize)
            }
        }

        fn coords(&self, index: usize) -> (i32, i32) {
            ((index as i32) % self.cols, (index as i32) / self.cols)
        }

        fn neighbors(&self, index: usize) -> Vec<usize> {
            let (col, row) = self.coords(index);
            let mut found = Vec::new();
            for dy in -1..=1 {
                for dx in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    if let Some(next) = self.index(col + dx, row + dy) {
                        found.push(next);
                    }
                }
            }
            found
        }

        fn next_u32(&mut self) -> u32 {
            let mut value = self.rng;
            value ^= value << 13;
            value ^= value >> 17;
            value ^= value << 5;
            self.rng = value;
            value
        }

        #[cfg(test)]
        fn from_layout(rows: &[&str]) -> Self {
            let height = rows.len() as i32;
            let width = rows[0].len() as i32;
            let mut game = Self::blank(width, height);
            game.placed = true;
            for (row, line) in rows.iter().enumerate() {
                for (col, ch) in line.chars().enumerate() {
                    let index = game.index(col as i32, row as i32).unwrap();
                    game.cells[index].mine = ch == '*';
                }
            }
            game.recount();
            game
        }
    }

    #[cfg(test)]
    mod tests {
        use super::Minesweeper;

        #[test]
        fn a_zero_opens_its_neighbors_and_stops_at_a_closed_number() {
            let mut game = Minesweeper::from_layout(&["*..", "...", "*.."]);
            game.open_at(2, 0);
            assert!(game.cell(2, 2).unwrap().open);
            assert_eq!(game.cell(1, 0).unwrap().adjacent, 1);
            assert!(game.cell(1, 0).unwrap().open);
            assert!(!game.cell(0, 1).unwrap().open);
            assert!(game.alive());
            assert!(!game.won());
        }

        #[test]
        fn opening_the_last_safe_cell_wins() {
            let mut game = Minesweeper::from_layout(&["***", "*.*", "***"]);
            game.open_at(1, 1);
            assert!(game.won());
            assert_eq!(game.cell(1, 1).unwrap().adjacent, 8);
        }

        #[test]
        fn a_flag_blocks_the_cell_under_it() {
            let mut game = Minesweeper::from_layout(&["...", "...", "..."]);
            game.flag_at(1, 1);
            assert!(game.cell(1, 1).unwrap().flag);
            game.open_at(1, 1);
            assert!(!game.cell(1, 1).unwrap().open);
            game.flag_at(1, 1);
            assert!(!game.cell(1, 1).unwrap().flag);
            assert_eq!(game.flag_count(), 0);
        }

        #[test]
        fn opening_a_mine_ends_the_game_without_opening_neighbors() {
            let mut game = Minesweeper::from_layout(&["*..", "...", "..."]);
            game.open_at(0, 0);
            assert!(!game.alive());
            assert_eq!(game.exploded(), Some((0, 0)));
            assert!(!game.cell(1, 0).unwrap().open);
            game.press("space");
            assert!(game.alive());
            assert!(!game.cell(0, 0).unwrap().open);
            assert_eq!(game.mine_count(), 0);
        }

        #[test]
        fn the_first_open_is_never_a_mine() {
            let mut game = Minesweeper::new();
            game.open_at(0, 0);
            assert!(game.alive());
            assert!(!game.cell(0, 0).unwrap().mine);
            assert!(game.cell(0, 0).unwrap().open);
            assert_eq!(game.mine_count(), 10);
        }

        #[test]
        fn a_click_lands_on_the_cell_under_it() {
            let game = Minesweeper::new();
            let (origin_x, origin_y, size) = game.frame(90.0, 90.0);
            assert!((size - 10.0).abs() < 0.01);
            assert_eq!(
                game.cell_at(origin_x + size * 0.5, origin_y + size * 1.5, 90.0, 90.0),
                Some((0, 1))
            );
            assert_eq!(game.cell_at(-1.0, -1.0, 90.0, 90.0), None);
        }

        #[test]
        fn escape_asks_the_host_to_quit() {
            let mut game = Minesweeper::new();
            assert!(!game.quit());
            game.press("escape");
            assert!(game.quit());
        }
    }
}

impl Game for model::Minesweeper {
    fn update(&mut self, _dt: f32) {}

    fn draw(&self, graphics: &mut Graphics<'_>) {
        paint(self, graphics);
    }

    fn keypressed(&mut self, key: &str) {
        self.press(key);
    }

    fn mousepressed(&mut self, button: i32, x: f32, y: f32, width: f32, height: f32) {
        let Some((col, row)) = self.cell_at(x, y, width, height) else {
            return;
        };
        if button == 2 {
            self.flag_at(col, row);
        } else {
            self.open_at(col, row);
        }
    }

    fn quit(&self) -> bool {
        model::Minesweeper::quit(self)
    }
}

fn paint(game: &model::Minesweeper, graphics: &mut Graphics<'_>) {
    let width = graphics.width();
    let height = graphics.height();
    if width < 1.0 || height < 1.0 {
        return;
    }
    graphics.set_color(18, 22, 28);
    graphics.rectangle("fill", 0.0, 0.0, width, height);
    let (origin_x, origin_y, size) = game.frame(width, height);
    let gap = if size > 8.0 { 1.0 } else { 0.0 };
    let exploded = game.exploded();
    for row in 0..game.rows() {
        for col in 0..game.cols() {
            let Some(cell) = game.cell(col, row) else {
                continue;
            };
            let x = origin_x + col as f32 * size + gap;
            let y = origin_y + row as f32 * size + gap;
            let side = (size - gap * 2.0).max(1.0);
            let boom = exploded == Some((col, row));
            if boom {
                graphics.set_color(176, 48, 48);
            } else if cell.open {
                graphics.set_color(28, 34, 42);
            } else {
                graphics.set_color(58, 72, 92);
            }
            graphics.rectangle("fill", x, y, side, side);
            if cell.flag || (game.won() && cell.mine) {
                graphics.set_color(220, 72, 64);
                graphics.rectangle(
                    "fill",
                    x + side * 0.35,
                    y + side * 0.2,
                    side * 0.18,
                    side * 0.55,
                );
            } else if cell.open && cell.adjacent > 0 {
                let (red, green, blue) = number_color(cell.adjacent);
                graphics.set_color(red, green, blue);
                graphics.print(&cell.adjacent.to_string(), x + side * 0.28, y + side * 0.22);
            } else if !game.alive() && cell.mine {
                graphics.set_color(24, 24, 28);
                graphics.circle(x + side / 2.0, y + side / 2.0, side * 0.22);
            }
        }
    }
    graphics.set_color(232, 224, 200);
    let left = game.mine_count() - game.flag_count();
    graphics.print(&format!("MINES {left}"), origin_x + 4.0, origin_y + 4.0);
    if !game.alive() || game.won() {
        graphics.set_color(250, 220, 140);
        let banner = if game.won() { "WIN" } else { "DEAD" };
        graphics.print(banner, origin_x + 4.0, origin_y + size);
        graphics.print("SPACE", origin_x + 4.0, origin_y + size + 16.0);
    }
}

fn number_color(adjacent: u8) -> (u8, u8, u8) {
    match adjacent {
        1 => (80, 160, 240),
        2 => (72, 176, 96),
        3 => (220, 72, 64),
        4 => (96, 96, 200),
        5 => (176, 96, 48),
        6 => (48, 160, 160),
        7 => (80, 80, 88),
        _ => (160, 160, 168),
    }
}

fn main() {
    model::Minesweeper::new().run(Args {
        name: "Minesweeper",
        title: "Minesweeper",
        width: 480,
        height: 480,
    });
}
