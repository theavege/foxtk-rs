use foxtk::prelude::*;

pub enum Msg {
    Check1Changed(bool),
    Check2Changed(bool),
}

#[derive(Default)]
pub struct CheckExample {
    check1: foxtk::CheckButton,
    check2: foxtk::CheckButton,
    status: foxtk::TextField,
}

impl Component for CheckExample {
    type Event = Msg;
    type State = (bool, bool);
    fn handle(msg: Self::Event, model: &mut Self::State, _: Sender<Self::Event>) -> bool {
        match msg {
            Msg::Check1Changed(checked) => model.0 = checked,
            Msg::Check2Changed(checked) => model.1 = checked,
        };
        true
    }
    fn update(&self, model: &Self::State) {
        self.status
            .set_text(&format!("Check1: {}, Check2: {}", model.0, model.1));
    }
    fn view(&mut self, parent: &impl WindowExt, sender: Sender<Self::Event>) {
        let vbox = foxtk::VerticalFrame::new(parent);
        self.check1 = foxtk::CheckButton::new(&vbox, "Option 1");
        self.check1.set_callback({
            let sender = sender.clone();
            move |check| {
                sender.send(Msg::Check1Changed(check.check())).unwrap();
                false
            }
        });
        self.check2 = foxtk::CheckButton::new(&vbox, "Option 2");
        self.check2.set_callback({
            let sender = sender.clone();
            move |check| {
                sender.send(Msg::Check2Changed(check.check())).unwrap();
                false
            }
        });
        self.status = foxtk::TextField::new(&vbox, 20);
    }
}
