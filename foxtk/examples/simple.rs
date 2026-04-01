mod models {
    #[derive(Default)]
    pub struct Model(i32);
    impl Model {
        pub fn value(&self) -> i32 {
            self.0
        }
        pub fn shift(&mut self, value: i32) {
            self.0 += value;
        }
    }
}

enum Msg {
    SetVal(i32),
}

use foxtk::{Component, Widget};

#[derive(Default)]
struct Simple(foxtk::TextField);

impl Component for Simple {
    type Event = Msg;
    type State = models::Model;
    fn handle(msg: Self::Event, model: &mut Self::State, _: foxtk::Sender<Self::Event>) -> bool {
        match msg {
            Msg::SetVal(val) => model.shift(val),
        };
        true
    }
    fn update(&self, model: &Self::State) {
        self.0.set_text(&model.value().to_string());
    }
    fn view(&mut self, parent: &foxtk::MainWindow, sender: foxtk::Sender<Self::Event>) {
        let vframe = foxtk::Frame::new(parent);
        foxtk::Button::new(&vframe, "+").set_callback({
            let sender = sender.clone();
            move |_| {
                sender.send(Msg::SetVal(1)).unwrap();
                true
            }
        });
        self.0 = foxtk::TextField::new(&vframe, 6);
        foxtk::Button::new(&vframe, "-").set_callback({
            let sender = sender.clone();
            move |_| {
                sender.send(Msg::SetVal(-1)).unwrap();
                true
            }
        });
    }
}

fn main() {
    Simple::run();
}
