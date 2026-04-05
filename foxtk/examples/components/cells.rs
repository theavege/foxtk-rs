use foxtk::prelude::*;

pub enum Msg {
    UpdateCell(usize, usize, String),
}

#[derive(Default)]
pub struct CellsState {
    cells: Vec<Vec<String>>,
}

impl CellsState {
    pub fn new() -> Self {
        let mut cells = vec![vec!["".to_string(); 3]; 3];
        cells[0][0] = "1".to_string();
        cells[0][1] = "2".to_string();
        cells[0][2] = "=A1+B1".to_string();
        Self { cells }
    }
    pub fn update_cell(&mut self, row: usize, col: usize, value: String) {
        if row < self.cells.len() && col < self.cells[row].len() {
            let is_formula = value.starts_with('=');
            self.cells[row][col] = value;
            if is_formula {
                let cell_value = &self.cells[row][col];
                // Simple formula evaluation for =A1+B1
                let parts: Vec<&str> = cell_value[1..].split('+').collect();
                if parts.len() == 2 {
                    if let (Some(r1), Some(c1)) = parse_cell(parts[0]) {
                        if let (Some(r2), Some(c2)) = parse_cell(parts[1]) {
                            if r1 < self.cells.len() && c1 < self.cells[r1].len() &&
                               r2 < self.cells.len() && c2 < self.cells[r2].len() {
                                if let (Ok(v1), Ok(v2)) = (
                                    self.cells[r1][c1].parse::<f64>(),
                                    self.cells[r2][c2].parse::<f64>()
                                ) {
                                    self.cells[row][col] = (v1 + v2).to_string();
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    pub fn cells(&self) -> &Vec<Vec<String>> {
        &self.cells
    }
}

fn parse_cell(s: &str) -> (Option<usize>, Option<usize>) {
    if s.len() == 2 {
        let col = s.chars().nth(0).unwrap() as usize - 'A' as usize;
        let row = s.chars().nth(1).unwrap() as usize - '1' as usize;
        (Some(row), Some(col))
    } else {
        (None, None)
    }
}

pub type CellsModel = CellsState;

#[derive(Default)]
pub struct CellsExample {
    table: Option<foxtk::Table>,
}

impl Component for CellsExample {
    type Event = Msg;
    type State = CellsModel;
    fn handle(msg: Self::Event, model: &mut Self::State, _: Sender<Self::Event>) -> bool {
        match msg {
            Msg::UpdateCell(r, c, v) => model.update_cell(r, c, v),
        };
        true
    }
    fn update(&self, model: &Self::State) {
        if let Some(ref table) = self.table {
            for (r, row) in model.cells().iter().enumerate() {
                for (c, cell) in row.iter().enumerate() {
                    table.set_item_text(r as i32, c as i32, cell);
                }
            }
        }
    }
    fn view(&mut self, parent: &impl WindowExt, _sender: Sender<Self::Event>) {
        foxtk::VerticalFrame::new(parent).inside(|vbox| {
            foxtk::Button::new(vbox, "7GUIs Cells");
            self.table = Some(foxtk::Table::new(vbox));
            if let Some(ref table) = self.table {
                table.set_table_size(3, 3);
                // Note: Table editing not implemented, so cells are read-only in this example
            }
        });
    }
}