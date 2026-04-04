mod models {
    #[derive(Default)]
    pub struct Model(i32);
    impl Model {
        pub fn value(&self) -> i32 {
            self.0
        }
        pub fn set(&mut self, value: i32) {
            self.0 = value;
        }
        pub fn shift(&mut self, value: i32) {
            self.0 += value;
        }
    }
}

enum Msg {
    SetVal(i32),
    SetValue(i32),
}

use foxtk::prelude::*;
mod components;

#[derive(Default)]
struct Simple(foxtk::TextField, foxtk::Spinner, foxtk::RangeSlider);

impl Component for Simple {
    type Event = Msg;
    type State = models::Model;
    fn handle(msg: Self::Event, model: &mut Self::State, _: foxtk::Sender<Self::Event>) -> bool {
        match msg {
            Msg::SetVal(val) => model.shift(val),
            Msg::SetValue(val) => model.set(val),
        };
        true
    }
    fn update(&self, model: &Self::State) {
        self.0.set_text(&model.value().to_string());
        self.1.set_value(model.value());
        self.2.set_value(model.value());
    }
    fn view(&mut self, parent: &impl WindowExt, sender: foxtk::Sender<Self::Event>) {
        let vbox = foxtk::VerticalFrame::new(parent);
        let hbox = foxtk::HorizontalFrame::new(&vbox);
        foxtk::Button::new(&hbox, "+").set_callback({
            let sender = sender.clone();
            move |_| {
                sender.send(Msg::SetVal(1)).unwrap();
                false
            }
        });
        self.0 = foxtk::TextField::new(&hbox, 6);
        self.1 = foxtk::Spinner::new(&hbox, 4);
        self.1.set_range(0, 100);
        self.1.set_increment(1);
        self.1.set_callback({
            let sender = sender.clone();
            move |spinner: foxtk::Spinner| {
                sender.send(Msg::SetValue(spinner.get_value())).unwrap();
                false
            }
        });
        self.2 = foxtk::RangeSlider::new(&hbox);
        self.2.set_range(0, 100);
        self.2.set_increment(1);
        self.2.set_callback({
            let sender = sender.clone();
            move |slider: foxtk::RangeSlider| {
                sender.send(Msg::SetValue(slider.get_value())).unwrap();
                false
            }
        });
        foxtk::Button::new(&hbox, "-").set_callback({
            let sender = sender.clone();
            move |_| {
                sender.send(Msg::SetVal(-1)).unwrap();
                false
            }
        });
        components::Converter::mount(&hbox);
        components::RadioExample::mount(&hbox);
        components::CheckExample::mount(&hbox);
    }
}

fn main() {
    Simple::run("Name", "Vendor", "Title");
}
