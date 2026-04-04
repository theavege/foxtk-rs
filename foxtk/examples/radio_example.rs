use foxtk::prelude::*;

enum Msg {
    RadioChanged(bool),
}

#[derive(Default)]
struct RadioExample {
    radio1: foxtk::RadioButton,
    radio2: foxtk::RadioButton,
    status: foxtk::TextField,
}

impl Component for RadioExample {
    type Event = Msg;
    type State = bool;
    fn handle(msg: Self::Event, model: &mut Self::State, _: foxtk::Sender<Self::Event>) -> bool {
        match msg {
            Msg::RadioChanged(selected) => *model = selected,
        };
        true
    }
    fn update(&self, model: &Self::State) {
        self.status.set_text(&format!("Radio 1 selected: {}", *model));
    }
    fn view(&mut self, parent: &impl WindowExt, sender: foxtk::Sender<Self::Event>) {
        let vbox = foxtk::VerticalFrame::new(parent);
        self.radio1 = foxtk::RadioButton::new(&vbox, "Option 1");
        self.radio1.set_callback({
            let sender = sender.clone();
            move |_| {
                sender.send(Msg::RadioChanged(true)).unwrap();
                false
            }
        });
        self.radio2 = foxtk::RadioButton::new(&vbox, "Option 2");
        self.radio2.set_callback({
            let sender = sender.clone();
            move |_| {
                sender.send(Msg::RadioChanged(false)).unwrap();
                false
            }
        });
        self.status = foxtk::TextField::new(&vbox, 20);
    }
}

fn main() {
    RadioExample::run("RadioButton Example", "foxtk", "Radio Buttons");
}