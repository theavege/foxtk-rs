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
            self.output.push_str(&format!(" {}<br>", self.current));
            let current: f64 = self.current.parse().unwrap();
            self.prev = match self.operation.as_str() {
                "/" => self.prev / current,
                "x" => self.prev * current,
                "+" => self.prev + current,
                "-" => self.prev - current,
                _ => self.prev / 100.0 * current,
            };
            self.output.push_str(&format!("    = {}<br>", self.prev));
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
    outp: foxtk::TextField,
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
        self.outp.set_value(&model.output.clone());
        self.prev.set_value(&model.prev.to_string());
        self.oper.set_value(&model.operation.clone());
        self.curr.set_value(&model.current.clone());
    }
    fn view(&mut self, parent: &impl CompositeExt, sender: Sender<Self::Event>) {
        efl::Box::new(parent)
            .with_homogeneous(false)
            .insert(|parent| {
                self.outp = efl::Entry::new(parent)
                    .with_editable(false)
                    .with_single_line(false)
                    .with_tooltip("Output");
                efl::Box::new(parent)
                    .with_horizontal(true)
                    .insert(|parent| {
                        self.oper = efl::Entry::new(parent)
                            .with_editable(false)
                            .with_single_line(false)
                            .with_tooltip("Operation");
                        efl::Box::new(parent).insert(|parent| {
                            self.prev = efl::Entry::new(parent)
                                .with_size(0, 45)
                                .with_editable(false)
                                .with_tooltip("Previos");
                            self.curr = efl::Entry::new(parent)
                                .with_size(0, 45)
                                .with_editable(false)
                                .with_tooltip("Current");
                        });
                    });
                for row in [
                    ["CE", "C", "%", "/"],
                    ["7", "8", "9", "x"],
                    ["4", "5", "6", "-"],
                    ["1", "2", "3", "+"],
                    ["0", ".", "<", "="],
                ] {
                    efl::Box::new(parent)
                        .with_weight(true, false)
                        .with_horizontal(true)
                        .insert(|parent| {
                            for cell in row {
                                efl::Button::new(parent)
                                    .with_size(90, 90)
                                    .with_text(cell)
                                    .with_tooltip(cell)
                                    .with_cursor("hand1")
                                    .on_clicked({
                                        let sender = sender.clone();
                                        move |wgt| {
                                            sender.send(Msg::Push(wgt.text())).unwrap();
                                        }
                                    });
                            }
                        });
                }
            });
    }
}
