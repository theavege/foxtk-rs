use foxtk::prelude::*;

#[derive(Clone, Copy)]
enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

pub struct Model {
    display: String,
    accumulator: Option<f64>,
    operator: Option<Operator>,
    replace_display: bool,
    error: Option<String>,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            display: "0".into(),
            accumulator: None,
            operator: None,
            replace_display: false,
            error: None,
        }
    }
}

impl Model {
    fn new() -> Self {
        Self::default()
    }

    fn press(&mut self, key: &str) {
        match key {
            "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => self.digit(key),
            "." => self.decimal_point(),
            "<-" => self.backspace(),
            "C" => *self = Self::new(),
            "CE" => {
                self.display = "0".into();
                self.error = None;
                self.replace_display = false;
            }
            "+/-" => self.toggle_sign(),
            "%" => self.percent(),
            "+" => self.set_operator(Operator::Add),
            "-" => self.set_operator(Operator::Subtract),
            "×" => self.set_operator(Operator::Multiply),
            "÷" => self.set_operator(Operator::Divide),
            "=" => self.equals(),
            _ => {}
        }
    }

    fn digit(&mut self, digit: &str) {
        if self.error.take().is_some() || self.replace_display {
            self.display = "0".into();
            self.replace_display = false;
        }
        if self.display == "0" && digit != "0" {
            self.display.clear();
        }
        if self.display.len() < 24 {
            self.display.push_str(digit);
        }
    }

    fn decimal_point(&mut self) {
        if self.error.take().is_some() || self.replace_display {
            self.display = "0".into();
            self.replace_display = false;
        }
        if !self.display.contains('.') {
            self.display.push('.');
        }
    }

    fn backspace(&mut self) {
        if self.error.take().is_some() || self.replace_display {
            self.display = "0".into();
            self.replace_display = false;
        } else {
            self.display.pop();
            if self.display.is_empty() || self.display == "-" {
                self.display = "0".into();
            }
        }
    }

    fn toggle_sign(&mut self) {
        if self.error.is_some() {
            return;
        }
        if let Some(stripped) = self.display.strip_prefix('-') {
            self.display = stripped.to_owned();
        } else if self.display != "0" {
            self.display.insert(0, '-');
        }
    }

    fn percent(&mut self) {
        if let Some(value) = self.current_value() {
            self.set_result(value / 100.0);
        }
    }

    fn set_operator(&mut self, operator: Operator) {
        let Some(current) = self.current_value() else {
            return;
        };
        if self.operator.is_some() && !self.replace_display {
            if !self.evaluate(current) {
                return;
            }
        } else if self.accumulator.is_none() {
            self.accumulator = Some(current);
        }
        self.operator = Some(operator);
        self.replace_display = true;
    }

    fn equals(&mut self) {
        if let (Some(_), Some(current)) = (self.operator, self.current_value())
            && self.evaluate(current)
        {
            self.operator = None;
            self.accumulator = None;
            self.replace_display = true;
        }
    }

    fn evaluate(&mut self, rhs: f64) -> bool {
        let Some(lhs) = self.accumulator else {
            return false;
        };
        let Some(operator) = self.operator else {
            return false;
        };
        let result = match operator {
            Operator::Add => lhs + rhs,
            Operator::Subtract => lhs - rhs,
            Operator::Multiply => lhs * rhs,
            Operator::Divide if rhs != 0.0 => lhs / rhs,
            Operator::Divide => {
                self.fail("Cannot divide by zero");
                return false;
            }
        };
        if !result.is_finite() {
            self.fail("Result is outside the supported range");
            return false;
        }
        self.display = format_number(result);
        self.accumulator = Some(result);
        self.replace_display = true;
        true
    }

    fn current_value(&mut self) -> Option<f64> {
        match self.display.parse::<f64>() {
            Ok(value) if value.is_finite() => Some(value),
            _ => {
                self.fail("Invalid number");
                None
            }
        }
    }

    fn set_result(&mut self, value: f64) {
        if value.is_finite() {
            self.display = format_number(value);
            self.replace_display = true;
        } else {
            self.fail("Result is outside the supported range");
        }
    }

    fn fail(&mut self, message: &str) {
        self.display = "Error".into();
        self.accumulator = None;
        self.operator = None;
        self.replace_display = true;
        self.error = Some(message.into());
    }
}

fn format_number(value: f64) -> String {
    let value = value.to_string();
    value
        .strip_suffix(".0")
        .map_or(value.clone(), str::to_owned)
}

pub enum Event {
    Press(String),
}

#[derive(Default)]
pub struct Calc {
    display: foxtk::TextField,
}

impl Component for Calc {
    type Event = Event;
    type State = Model;

    fn handle(event: Self::Event, model: &mut Self::State, _: Sender<Self::Event>) -> bool {
        match event {
            Event::Press(key) => model.press(&key),
        }
        true
    }

    fn update(&self, model: &Self::State) {
        let display = model.error.as_deref().unwrap_or(&model.display);
        if self.display.text() != display {
            self.display.set_text(display);
        }
    }

    fn view(&mut self, parent: &impl CompositeExt, sender: Sender<Self::Event>) {
        foxtk::VerticalFrame::new(parent)
            .with_layout(Layout::Fill)
            .with_spacing(4)
            .inside(|parent| {
                self.display = foxtk::TextField::new(parent).with_editable(false);
                for row in [
                    ["CE", "C", "<-", "÷"],
                    ["7", "8", "9", "×"],
                    ["4", "5", "6", "-"],
                    ["1", "2", "3", "+"],
                    ["+/-", "0", ".", "%"],
                    ["=", "", "", ""],
                ] {
                    foxtk::HorizontalFrame::new(parent)
                        .with_layout(Layout::FillX)
                        .with_spacing(4)
                        .inside(|parent| {
                            for key in row.into_iter().filter(|key| !key.is_empty()) {
                                foxtk::Button::new(parent, key)
                                    .with_layout(Layout::Fill)
                                    .with_callback({
                                        let sender = sender.clone();
                                        move |_| {
                                            if let Err(error) =
                                                sender.send(Event::Press(key.to_owned()))
                                            {
                                                eprintln!("Calculator event was dropped: {error}");
                                            }
                                            false
                                        }
                                    });
                            }
                        });
                }
            });
    }
}

#[cfg(test)]
mod tests {
    use super::Model;

    #[test]
    fn evaluates_chained_operations_left_to_right() {
        let mut model = Model::new();
        for key in ["8", "+", "2", "×", "3", "="] {
            model.press(key);
        }
        assert_eq!(model.display, "30");
    }

    #[test]
    fn division_by_zero_reports_an_error_without_panicking() {
        let mut model = Model::new();
        for key in ["9", "÷", "0", "="] {
            model.press(key);
        }
        assert_eq!(model.display, "Error");
        assert_eq!(model.error.as_deref(), Some("Cannot divide by zero"));
    }

    #[test]
    fn clear_entry_keeps_the_pending_operation() {
        let mut model = Model::new();
        for key in ["5", "+", "CE", "2", "="] {
            model.press(key);
        }
        assert_eq!(model.display, "7");
    }
}
