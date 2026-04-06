use foxtk::prelude::*;

pub enum Msg {
    Set(i32),
}

#[derive(Default)]
pub struct Selectors {
    list: foxtk::ListBox,
    combo: foxtk::ComboBox,
}

impl Component for Selectors {
    type Event = Msg;
    type State = i32;
    fn update(&self, model: &Self::State) {
        self.list.set_current_item(*model);
        self.combo.set_current_item(*model);
    }
    fn handle(msg: Self::Event, model: &mut Self::State, _: Sender<Self::Event>) -> bool {
        match msg {
            Msg::Set(value) => *model = value,
        };
        true
    }
    fn view(&mut self, parent: &impl CompositeExt, sender: Sender<Self::Event>) {
        foxtk::HorizontalFrame::new(parent).inside(|hbox| {
            let items = ["Item 1", "Item 2", "Item 3", "Item 4", "Item 5"];
            self.list = foxtk::ListBox::new(hbox).with_items(&items).with_callback({
                let sender = sender.clone();
                move |wgt| {
                    sender.send(Msg::Set(wgt.current_item())).unwrap();
                    false
                }
            });
            self.combo = foxtk::ComboBox::new(hbox, 6)
                .with_items(&items)
                .with_callback({
                    let sender = sender.clone();
                    move |wgt| {
                        sender.send(Msg::Set(wgt.current_item())).unwrap();
                        false
                    }
                });
        });
    }
}
