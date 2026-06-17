#![forbid(unsafe_code)]

use foxtk::prelude::*;
use std::sync::mpsc::Sender;

pub enum Msg {
    Press(String),
}

#[derive(Default)]
pub struct Calculator {
    history: foxtk::Text,
    display: foxtk::TextField,
}

#[derive(Default)]
pub struct CalcState {
    current: String,
    operand: Option<String>,
    operator: Option<char>,
    history: String,
}

impl CalcState {
    fn push(&mut self, value: &str) {
        match value {
            "C" => {
                self.current = String::from("0");
                self.operator = None;
                self.operand = None;
                self.history.clear();
            }
            "CE" => {
                self.current = String::from("0");
            }
            "<" => {
                if self.current.len() > 1 {
                    self.current.pop();
                } else {
                    self.current = String::from("0");
                }
            }
            "+/-" => {
                if self.current.starts_with('-') {
                    self.current.remove(0);
                } else if self.current != "0" {
                    self.current.insert(0, '-');
                }
            }
            "%" => {
                if let Ok(value) = self.current.parse::<f64>() {
                    self.current = (value / 100.0).to_string();
                }
            }
            "." => {
                if !self.current.contains('.') {
                    self.current.push('.');
                }
            }
            "=" => {
                self.compute();
                self.operator = None;
                self.operand = None;
            }
            op if ["/", "x", "+", "-"].contains(&op) => {
                if self.operator.is_some() {
                    self.compute();
                }
                self.operand = Some(self.current.clone());
                self.operator = op.chars().next();
                self.history = format!("{} {}", self.current, op);
                self.current = String::from("0");
            }
            digit => {
                if self.current == "0" {
                    self.current = digit.to_string();
                } else {
                    self.current.push_str(digit);
                }
            }
        }
    }

    fn compute(&mut self) {
        if let (Some(op), Some(lhs)) = (self.operator, self.operand.clone()) {
            if let Ok(a) = lhs.parse::<f64>() {
                if let Ok(b) = self.current.parse::<f64>() {
                    let result = match op {
                        '/' => a / b,
                        'x' => a * b,
                        '+' => a + b,
                        '-' => a - b,
                        _ => b,
                    };
                    self.history = format!("{} {} {} =", lhs, op, self.current);
                    self.current = result.to_string();
                }
            }
        }
    }
}

impl Component for Calculator {
    type Event = Msg;
    type State = CalcState;

    fn handle(msg: Self::Event, model: &mut Self::State, _: Sender<Self::Event>) -> bool {
        match msg {
            Msg::Press(value) => model.push(&value),
        }
        true
    }

    fn update(&self, model: &Self::State) {
        self.display.set_text(&model.current);
        self.history.set_text(&model.history);
    }

    fn view(&mut self, prt: &impl CompositeExt, sender: Sender<Self::Event>) {
        let outer = foxtk::VerticalFrame::new(prt).with_layout(Layout::Fill);

        outer.inside(|prt| {
            self.history = foxtk::Text::new(prt)
                .with_editable(false)
                .with_height(60)
                .with_text("Calculator");

            self.display = foxtk::TextField::new(prt)
                .with_layout(Layout::FillX)
                .with_width(300)
                .with_text("0")
                .with_editable(false);

            let rows: &[&[&str]] = &[
                &["CE", "C", "<", "/"],
                &["7", "8", "9", "x"],
                &["4", "5", "6", "-"],
                &["1", "2", "3", "+"],
                &["0", ".", "+/-", "="],
                &["%", "", "", ""],
            ];

            for row in rows {
                foxtk::HorizontalFrame::new(prt).inside(|prt| {
                    for &cell in *row {
                        if cell.is_empty() {
                            foxtk::Spring::new(prt);
                        } else {
                            foxtk::Button::new(prt, cell)
                                .with_layout(Layout::Fill)
                                .set_callback({
                                    let sender = sender.clone();
                                    move |wgt| {
                                        sender.send(Msg::Press(wgt.text())).unwrap();
                                        false
                                    }
                                });
                        }
                    }
                });
            }
        });
    }
}

fn main() {
    Calculator::run("FOX Calculator", "FOX", "Calculator", 320, 420);
}
