use foxtk::prelude::*;

pub enum Msg {
    Set(i32),
}

#[derive(Default)]
pub struct Selectors {
    list: foxtk::List,
    listbox: foxtk::ListBox,
    combo: foxtk::ComboBox,
}

impl Component for Selectors {
    type Event = Msg;
    type State = i32;
    fn update(&self, model: &Self::State) {
        self.list.update(*model);
        self.listbox.update(*model);
        self.combo.update(*model);
    }
    fn handle(msg: Self::Event, model: &mut Self::State, _: Sender<Self::Event>) -> bool {
        match msg {
            Msg::Set(value) => *model = value,
        };
        true
    }
    fn view(&mut self, prt: &impl CompositeExt, sender: Sender<Self::Event>) {
        foxtk::GroupBox::new(prt, "Selectors")
            .with_frame(FrameStyle::Line)
            .with_layout(Layout::Fill)
            .inside(|prt| {
                let items = ["Item 1", "Item 2", "Item 3", "Item 4", "Item 5"];
                self.listbox = foxtk::ListBox::new(prt).with_items(&items).with_callback({
                    let sender = sender.clone();
                    move |wgt| {
                        sender.send(Msg::Set(wgt.current_item())).unwrap();
                        false
                    }
                });
                self.combo = foxtk::ComboBox::new(prt, 6)
                    .with_items(&items)
                    .with_callback({
                        let sender = sender.clone();
                        move |wgt| {
                            sender.send(Msg::Set(wgt.current_item())).unwrap();
                            false
                        }
                    });
                self.list = foxtk::List::new(prt)
                    .with_items(&items)
                    .with_layout(Layout::Fill)
                    .with_callback({
                        let sender = sender.clone();
                        move |wgt| {
                            if wgt.has_focus() {
                                sender.send(Msg::Set(wgt.current_item())).unwrap();
                            }
                            false
                        }
                    });
            });
    }
}
