mod models {
    pub struct Model {
        pub start: String,
        pub back: String,
        pub flight: bool,
    }
    impl Default for Model {
        fn default() -> Self {
            let current_date = chrono::offset::Local::now()
                .naive_local()
                .date()
                .format("%Y-%m-%d")
                .to_string();
            Self {
                start: current_date.clone(),
                back: current_date,
                flight: false,
            }
        }
    }
}

use foxtk::prelude::*;

pub enum Msg {
    Book,
    Flight(bool),
    Start(String),
    Back(String),
}

#[derive(Default)]
pub struct Booker {
    start: foxtk::TextField,
    back: foxtk::TextField,
    flight: foxtk::ListBox,
    book: foxtk::Button,
}

impl Component for Booker {
    type Event = Msg;
    type State = models::Model;
    fn update(&self, model: &Self::State) {
        self.flight.update(model.flight as i32);
        self.start.update(&model.start);
        self.back.set_enable(model.flight);
        self.back.update(&model.back);
    }
    fn handle(msg: Self::Event, model: &mut Self::State, _: Sender<Self::Event>) -> bool {
        match msg {
            Msg::Start(value) => {
                model.start = value;
                false
            }
            Msg::Back(value) => {
                model.back = value;
                false
            }
            Msg::Flight(value) => {
                model.flight = value;
                true
            }
            Msg::Book => true,
        };
        true
    }
    fn view(&mut self, prt: &impl CompositeExt, sender: Sender<Self::Event>) {
        const WIDTH: i32 = 100;
        foxtk::GroupBox::new(prt, "Booker").inside(|prt| {
            foxtk::HorizontalFrame::new(prt).inside(|prt| {
                foxtk::Label::new(prt, "Flight").with_width(WIDTH);
                self.flight = foxtk::ListBox::new(prt)
                    .with_items(&["One-way", "Return"])
                    .with_num_visible(2)
                    .with_callback({
                        let sender = sender.clone();
                        move |wgt| {
                            sender.send(Msg::Flight(wgt.current_item() != 0)).unwrap();
                            false
                        }
                    });
            });
            foxtk::HorizontalFrame::new(prt).inside(|prt| {
                foxtk::Label::new(prt, "Departure data").with_width(WIDTH);
                self.start = foxtk::TextField::new(prt).with_callback({
                    let sender = sender.clone();
                    move |wgt| {
                        if wgt.has_focus() {
                            if chrono::NaiveDate::parse_from_str(&wgt.text(), "%Y-%m-%d").is_ok() {
                                sender.send(Msg::Start(wgt.text())).unwrap();
                            } else {
                                wgt.message(MessageBox::Ok, "ERROR", Message::Warning);
                            }
                        }
                        false
                    }
                });
            });
            foxtk::HorizontalFrame::new(prt).with_frame(FrameStyle::Thick).inside(|prt| {
                foxtk::Label::new(prt, "Return data")
                    .with_width(WIDTH)
                    .set_text_color(Color::from_rgb(108, 113, 196));
                self.back = foxtk::TextField::new(prt)
                    .with_selector(Selector::COMMAND)
                    .with_callback({
                        let sender = sender.clone();
                        move |wgt| {
                            if wgt.has_focus() {
                                if chrono::NaiveDate::parse_from_str(&wgt.text(), "%Y-%m-%d")
                                    .is_ok()
                                {
                                    sender.send(Msg::Back(wgt.text())).unwrap();
                                } else {
                                    wgt.message(MessageBox::Ok, "ERROR", Message::Warning);
                                }
                            }
                            false
                        }
                    });
            });
            self.book = foxtk::Button::new(prt, "Book").with_width(WIDTH).with_callback({
                let sender = sender.clone();
                move |wgt| {
                    if wgt.has_focus() {
                        sender.send(Msg::Book).unwrap();
                    }
                    false
                }
            });
        });
    }
}
