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
    fn handle(
        msg: Self::Event,
        model: &mut Self::State,
        _: Sender<Self::Event>,
    ) -> ComponentOutcome {
        match msg {
            Msg::Start(value) => {
                model.start = value;
                ComponentOutcome::Changed
            }
            Msg::Back(value) => {
                model.back = value;
                ComponentOutcome::Changed
            }
            Msg::Flight(value) => {
                model.flight = value;
                ComponentOutcome::Changed
            }
            Msg::Book => ComponentOutcome::Changed,
        }
    }
    fn new(prt: &impl CompositeExt, sender: Sender<Self::Event>) -> Self {
        const WIDTH: i32 = 100;
        let group = foxtk::GroupBox::new(prt, "Booker");
        let flight_row = foxtk::HorizontalFrame::new(&group);
        foxtk::Label::new(&flight_row, "Flight").with_width(WIDTH);
        let flight = foxtk::ListBox::new(&flight_row)
            .with_items(&["One-way", "Return"])
            .with_num_visible(2)
            .with_callback({
                let sender = sender.clone();
                move |wgt| {
                    sender.send(Msg::Flight(wgt.current_item() != 0)).unwrap();
                    false
                }
            });
        let start_row = foxtk::HorizontalFrame::new(&group);
        foxtk::Label::new(&start_row, "Departure data").with_width(WIDTH);
        let start = foxtk::TextField::new(&start_row).with_callback({
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
        let back_row = foxtk::HorizontalFrame::new(&group).with_frame(FrameStyle::Thick);
        foxtk::Label::new(&back_row, "Return data")
            .with_width(WIDTH)
            .set_text_color(Color::from_rgb(108, 113, 196));
        let back = foxtk::TextField::new(&back_row)
            .with_selector(Selector::COMMAND)
            .with_callback({
                let sender = sender.clone();
                move |wgt| {
                    if wgt.has_focus() {
                        if chrono::NaiveDate::parse_from_str(&wgt.text(), "%Y-%m-%d").is_ok() {
                            sender.send(Msg::Back(wgt.text())).unwrap();
                        } else {
                            wgt.message(MessageBox::Ok, "ERROR", Message::Warning);
                        }
                    }
                    false
                }
            });
        let book = foxtk::Button::new(&group, "Book")
            .with_width(WIDTH)
            .with_callback({
                let sender = sender.clone();
                move |wgt| {
                    if wgt.has_focus() {
                        sender.send(Msg::Book).unwrap();
                    }
                    false
                }
            });
        Self {
            start,
            back,
            flight,
            book,
        }
    }
}
