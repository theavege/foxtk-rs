mod models {
    #[derive(Default)]
    pub struct Calc {
        pub prev: f64,
        pub operation: String,
        pub current: String,
        pub output: String,
    }

    impl Calc {
        pub fn click(&mut self, value: &str) {
            match value {
                "/" | "x" | "+" | "-" | "%" => {
                    if self.current != "0" {
                        if self.operation.is_empty() {
                            self.prev = self.current.parse().unwrap();
                        } else {
                            self.equil();
                        }
                        self.output.push_str(&format!("{} {}", self.prev, value));
                        self.operation = value.to_string();
                        self.current = String::from("0");
                    }
                }
                "=" => {
                    if !self.operation.is_empty() {
                        self.equil();
                        self.operation.clear();
                    }
                }
                "CE" => {
                    self.output.clear();
                    self.operation.clear();
                    self.current = String::from("0");
                    self.prev = 0f64;
                }
                "@<-" => {
                    let label = self.current.clone();
                    self.current = if label.len() > 1 {
                        String::from(&label[..label.len() - 1])
                    } else {
                        String::from("0")
                    };
                }
                "C" => self.current = String::from("0"),
                "." => {
                    if !self.current.contains('.') {
                        self.current.push('.');
                    }
                }
                _ => {
                    if self.current == "0" {
                        self.current.clear();
                    }
                    self.current.push_str(value);
                }
            };
        }
        fn equil(&mut self) {
            self.output.push_str(&format!(" {}\n", self.current));
            let current: f64 = self.current.parse().unwrap();
            self.prev = match self.operation.as_str() {
                "/" => self.prev / current,
                "x" => self.prev * current,
                "+" => self.prev + current,
                "-" => self.prev - current,
                _ => self.prev / 100.0 * current,
            };
            self.output.push_str(&format!("    = {}\n", self.prev));
            self.current = String::from("0");
        }
    }
}

use foxtk::prelude::*;

pub enum Msg {
    Push(String),
}

#[derive(Default)]
pub struct Calc {
    outp: foxtk::Text,
    prev: foxtk::TextField,
    oper: foxtk::TextField,
    curr: foxtk::TextField,
}

impl Component for Calc {
    type Event = Msg;
    type State = models::Calc;
    fn handle(msg: Self::Event, model: &mut Self::State, _: Sender<Self::Event>) -> bool {
        match msg {
            Msg::Push(value) => model.click(&value),
        };
        true
    }
    fn update(&self, model: &Self::State) {
        self.outp.update(&model.output);
        self.prev.update(&model.prev.to_string());
        self.oper.update(&model.operation);
        self.curr.update(&model.current);
    }
    fn view(&mut self, prt: &impl CompositeExt, sender: Sender<Self::Event>) {
        foxtk::VerticalFrame::new(prt).inside(|prt| {
            self.outp = foxtk::Text::new(prt)
                .with_font("cascadia mono", 12)
                .with_editable(false);
            foxtk::HorizontalFrame::new(prt).inside(|prt| {
                self.oper = foxtk::TextField::new(prt)
                    .with_width(60)
                    .with_layout(Layout::FillY)
                    .with_editable(false);
                foxtk::VerticalFrame::new(prt).inside(|prt| {
                    self.prev = foxtk::TextField::new(prt).with_editable(false);
                    self.curr = foxtk::TextField::new(prt).with_editable(false);
                });
            });
            for row in [
                ["CE", "C", "%", "/"],
                ["7", "8", "9", "x"],
                ["4", "5", "6", "-"],
                ["1", "2", "3", "+"],
                ["0", ".", "<", "="],
            ] {
                foxtk::HorizontalFrame::new(prt).inside(|prt| {
                    for cell in row {
                        foxtk::Button::new(prt, cell)
                            .with_layout(Layout::Fill)
                            .with_callback({
                                let sender = sender.clone();
                                move |wgt| {
                                    sender.send(Msg::Push(wgt.text())).unwrap();
                                    false
                                }
                            });
                    }
                });
            }
        });
    }
}
