//! 7GUIs cells: a scrollable sheet, A to Z and 0 to 99.
//!
//! A cell holds a number or a formula that starts with `=`. Editing finishes
//! on the table's own cell editor. Only cells that depend on the edit are
//! recomputed.

mod models {
    use std::collections::VecDeque;

    pub const ROWS: usize = 100;
    pub const COLS: usize = 26;
    const LEN: usize = ROWS * COLS;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum EvalError {
        Bad,
        Cycle,
    }

    #[derive(Clone, Debug)]
    pub struct Sheet {
        formula: Vec<String>,
        display: Vec<String>,
        number: Vec<f64>,
        deps: Vec<Vec<usize>>,
        generation: Vec<u32>,
    }

    impl Default for Sheet {
        fn default() -> Self {
            Self {
                formula: vec![String::new(); LEN],
                display: vec![String::new(); LEN],
                number: vec![0.0; LEN],
                deps: vec![Vec::new(); LEN],
                generation: vec![0; LEN],
            }
        }
    }

    impl Sheet {
        pub fn index(row: usize, col: usize) -> usize {
            row * COLS + col
        }

        pub fn formula(&self, row: usize, col: usize) -> &str {
            &self.formula[Self::index(row, col)]
        }

        pub fn display(&self, row: usize, col: usize) -> &str {
            &self.display[Self::index(row, col)]
        }

        pub fn generation(&self, row: usize, col: usize) -> u32 {
            self.generation[Self::index(row, col)]
        }

        pub fn set_formula(&mut self, row: usize, col: usize, text: String) {
            let index = Self::index(row, col);
            if self.formula[index] == text {
                return;
            }
            for source in references(&self.formula[index]) {
                self.deps[source].retain(|dep| *dep != index);
            }
            self.formula[index] = text;
            for source in references(&self.formula[index]) {
                if !self.deps[source].contains(&index) {
                    self.deps[source].push(index);
                }
            }
            self.propagate(index);
        }

        fn propagate(&mut self, start: usize) {
            let mut queue = VecDeque::from([start]);
            let mut queued = vec![false; LEN];
            queued[start] = true;
            let mut steps = 0;
            while let Some(index) = queue.pop_front() {
                queued[index] = false;
                steps += 1;
                if steps > LEN {
                    break;
                }
                let (number, display) = self.presented(index);
                if number != self.number[index] || display != self.display[index] {
                    self.number[index] = number;
                    self.display[index] = display;
                    self.generation[index] = self.generation[index].saturating_add(1);
                    for dep in self.deps[index].clone() {
                        if !queued[dep] {
                            queued[dep] = true;
                            queue.push_back(dep);
                        }
                    }
                }
            }
        }

        fn presented(&self, index: usize) -> (f64, String) {
            if self.formula[index].trim().is_empty() {
                return (0.0, String::new());
            }
            match self.value(index, &mut Vec::new()) {
                Ok(number) => (number, format_number(number)),
                Err(EvalError::Cycle) => (0.0, "#CYCLE".to_string()),
                Err(EvalError::Bad) => (0.0, "#ERR".to_string()),
            }
        }

        fn value(&self, index: usize, visiting: &mut Vec<usize>) -> Result<f64, EvalError> {
            if visiting.contains(&index) {
                return Err(EvalError::Cycle);
            }
            let formula = self.formula[index].trim();
            if formula.is_empty() {
                return Ok(0.0);
            }
            visiting.push(index);
            let result = if let Some(body) = formula.strip_prefix('=') {
                evaluate(body, &mut |cell| self.value(cell, visiting))
            } else {
                formula.parse::<f64>().map_err(|_| EvalError::Bad)
            };
            visiting.pop();
            result
        }
    }

    fn format_number(value: f64) -> String {
        if !value.is_finite() {
            return "#ERR".to_string();
        }
        if (value - value.round()).abs() < 1e-9 && value.abs() < 1e12 {
            return format!("{}", value.round() as i64);
        }
        let mut text = format!("{value:.6}");
        while text.contains('.') && (text.ends_with('0') || text.ends_with('.')) {
            text.pop();
        }
        text
    }

    fn references(formula: &str) -> Vec<usize> {
        let Some(body) = formula.trim().strip_prefix('=') else {
            return Vec::new();
        };
        let bytes = body.as_bytes();
        let mut index = 0;
        let mut found = Vec::new();
        while index < bytes.len() {
            if bytes[index].is_ascii_alphabetic() {
                let column = (bytes[index].to_ascii_uppercase() - b'A') as usize;
                let start = index;
                index += 1;
                let digits = index;
                while index < bytes.len() && bytes[index].is_ascii_digit() {
                    index += 1;
                }
                if column < COLS && digits < index {
                    if let Ok(row) = std::str::from_utf8(&bytes[digits..index])
                        .unwrap_or("")
                        .parse::<usize>()
                    {
                        if row < ROWS {
                            let cell = row * COLS + column;
                            if !found.contains(&cell) {
                                found.push(cell);
                            }
                            continue;
                        }
                    }
                }
                index = start + 1;
                continue;
            }
            index += 1;
        }
        found
    }

    fn evaluate(
        body: &str,
        cell: &mut impl FnMut(usize) -> Result<f64, EvalError>,
    ) -> Result<f64, EvalError> {
        let mut parser = Parser {
            bytes: body.as_bytes(),
            index: 0,
        };
        let value = parser.expr(cell)?;
        parser.skip();
        if parser.index != parser.bytes.len() {
            return Err(EvalError::Bad);
        }
        Ok(value)
    }

    struct Parser<'a> {
        bytes: &'a [u8],
        index: usize,
    }

    impl Parser<'_> {
        fn skip(&mut self) {
            while self.index < self.bytes.len() && self.bytes[self.index].is_ascii_whitespace() {
                self.index += 1;
            }
        }

        fn eat(&mut self, byte: u8) -> bool {
            self.skip();
            if self.index < self.bytes.len() && self.bytes[self.index] == byte {
                self.index += 1;
                true
            } else {
                false
            }
        }

        fn expr(
            &mut self,
            cell: &mut impl FnMut(usize) -> Result<f64, EvalError>,
        ) -> Result<f64, EvalError> {
            let mut value = self.term(cell)?;
            loop {
                if self.eat(b'+') {
                    value += self.term(cell)?;
                } else if self.eat(b'-') {
                    value -= self.term(cell)?;
                } else {
                    break;
                }
            }
            Ok(value)
        }

        fn term(
            &mut self,
            cell: &mut impl FnMut(usize) -> Result<f64, EvalError>,
        ) -> Result<f64, EvalError> {
            let mut value = self.unary(cell)?;
            loop {
                if self.eat(b'*') {
                    value *= self.unary(cell)?;
                } else if self.eat(b'/') {
                    let divisor = self.unary(cell)?;
                    if divisor == 0.0 {
                        return Err(EvalError::Bad);
                    }
                    value /= divisor;
                } else {
                    break;
                }
            }
            Ok(value)
        }

        fn unary(
            &mut self,
            cell: &mut impl FnMut(usize) -> Result<f64, EvalError>,
        ) -> Result<f64, EvalError> {
            if self.eat(b'+') {
                self.unary(cell)
            } else if self.eat(b'-') {
                Ok(-self.unary(cell)?)
            } else {
                self.primary(cell)
            }
        }

        fn primary(
            &mut self,
            cell: &mut impl FnMut(usize) -> Result<f64, EvalError>,
        ) -> Result<f64, EvalError> {
            self.skip();
            if self.eat(b'(') {
                let value = self.expr(cell)?;
                if !self.eat(b')') {
                    return Err(EvalError::Bad);
                }
                return Ok(value);
            }
            if self.index < self.bytes.len() && self.bytes[self.index].is_ascii_alphabetic() {
                let column = (self.bytes[self.index].to_ascii_uppercase() - b'A') as usize;
                if column >= COLS {
                    return Err(EvalError::Bad);
                }
                self.index += 1;
                let digits = self.index;
                while self.index < self.bytes.len() && self.bytes[self.index].is_ascii_digit() {
                    self.index += 1;
                }
                if digits == self.index {
                    return Err(EvalError::Bad);
                }
                let row = std::str::from_utf8(&self.bytes[digits..self.index])
                    .unwrap_or("")
                    .parse::<usize>()
                    .map_err(|_| EvalError::Bad)?;
                if row >= ROWS {
                    return Err(EvalError::Bad);
                }
                return cell(row * COLS + column);
            }
            self.number()
        }

        fn number(&mut self) -> Result<f64, EvalError> {
            let start = self.index;
            while self.index < self.bytes.len()
                && (self.bytes[self.index].is_ascii_digit() || self.bytes[self.index] == b'.')
            {
                self.index += 1;
            }
            if start == self.index {
                return Err(EvalError::Bad);
            }
            std::str::from_utf8(&self.bytes[start..self.index])
                .unwrap_or("")
                .parse()
                .map_err(|_| EvalError::Bad)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::Sheet;

        #[test]
        fn a_formula_updates_only_the_cells_that_depend_on_it() {
            let mut sheet = Sheet::default();
            sheet.set_formula(0, 0, "2".into());
            sheet.set_formula(0, 1, "=A0+3".into());
            sheet.set_formula(0, 2, "=B0*2".into());
            sheet.set_formula(1, 0, "9".into());
            assert_eq!(sheet.display(0, 0), "2");
            assert_eq!(sheet.display(0, 1), "5");
            assert_eq!(sheet.display(0, 2), "10");
            let unrelated = sheet.generation(1, 0);
            sheet.set_formula(0, 0, "4".into());
            assert_eq!(sheet.display(0, 0), "4");
            assert_eq!(sheet.display(0, 1), "7");
            assert_eq!(sheet.display(0, 2), "14");
            assert_eq!(sheet.generation(1, 0), unrelated);
        }

        #[test]
        fn a_cycle_is_reported_and_does_not_run_forever() {
            let mut sheet = Sheet::default();
            sheet.set_formula(0, 0, "=B0".into());
            sheet.set_formula(0, 1, "=A0".into());
            assert_eq!(sheet.display(0, 0), "#CYCLE");
            assert_eq!(sheet.display(0, 1), "#CYCLE");
            sheet.set_formula(2, 0, "=1/0".into());
            assert_eq!(sheet.display(2, 0), "#ERR");
        }
    }
}

use foxtk::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

pub enum Msg {
    Set { row: i32, col: i32, text: String },
}

pub struct Cells {
    table: foxtk::Table,
    sheet: Rc<RefCell<models::Sheet>>,
    shown: RefCell<Vec<String>>,
}

impl Default for Cells {
    fn default() -> Self {
        Self {
            table: foxtk::Table::default(),
            sheet: Rc::new(RefCell::new(models::Sheet::default())),
            shown: RefCell::new(Vec::new()),
        }
    }
}

impl Component for Cells {
    type Event = Msg;
    type State = models::Sheet;

    fn handle(
        msg: Self::Event,
        model: &mut Self::State,
        _sender: Sender<Self::Event>,
    ) -> ComponentOutcome {
        match msg {
            Msg::Set { row, col, text } => {
                if row < 0 || col < 0 {
                    return ComponentOutcome::Unchanged;
                }
                model.set_formula(row as usize, col as usize, text);
            }
        }
        ComponentOutcome::Changed
    }

    fn update(&self, model: &Self::State) {
        *self.sheet.borrow_mut() = model.clone();
        let writes = {
            let mut shown = self.shown.borrow_mut();
            if shown.len() != models::ROWS * models::COLS {
                shown.resize(models::ROWS * models::COLS, String::new());
            }
            let mut writes = Vec::new();
            for row in 0..models::ROWS {
                for col in 0..models::COLS {
                    let index = models::Sheet::index(row, col);
                    let display = model.display(row, col);
                    if shown[index] != display {
                        if !display.is_empty() || !shown[index].is_empty() {
                            writes.push((row as i32, col as i32, display.to_string()));
                        }
                        shown[index] = display.to_string();
                    }
                }
            }
            writes
        };
        for (row, col, text) in writes {
            self.table.set_item_text(row, col, &text);
        }
    }

    fn view(&mut self, parent: &impl CompositeExt, sender: Sender<Self::Event>) {
        self.table = foxtk::Table::new(parent);
        self.table.set_layout(Layout::Fill);
        self.table
            .set_table_size(models::ROWS as i32, models::COLS as i32);
        self.table.set_row_header_width(36);
        for column in 0..models::COLS {
            let name = ((b'A' + column as u8) as char).to_string();
            self.table.set_column_text(column as i32, &name);
            self.table.set_column_width(column as i32, 64);
        }
        for row in 0..models::ROWS {
            self.table.set_row_text(row as i32, &row.to_string());
        }
        let sheet = self.sheet.clone();
        let table = self.table.alias();
        table.with_callback(move |table| {
            let row = table.current_row();
            let col = table.current_column();
            if row < 0 || col < 0 {
                return false;
            }
            let text = table.item_text(row, col);
            let guard = sheet.borrow();
            let formula = guard.formula(row as usize, col as usize).to_string();
            let display = guard.display(row as usize, col as usize).to_string();
            drop(guard);
            if text == display && formula != display {
                table.set_item_text(row, col, &formula);
                return false;
            }
            if text != formula {
                let _ = sender.send(Msg::Set { row, col, text });
            } else if text != display {
                table.set_item_text(row, col, &display);
            }
            false
        });
    }
}
