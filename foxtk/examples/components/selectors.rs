use foxtk::prelude::*;

pub enum Msg {
    Set(i32),
}

pub struct Selectors {
    list: foxtk::List,
    listbox: foxtk::ListBox,
}

impl Component for Selectors {
    type Event = Msg;
    type State = i32;
    fn update(&self, model: &Self::State) {
        self.listbox.set_current_item(*model);
        self.list.set_current_item(*model);
    }
    fn handle(
        msg: Self::Event,
        model: &mut Self::State,
        _: Sender<Self::Event>,
    ) -> ComponentOutcome {
        match msg {
            Msg::Set(value) => *model = value,
        };
        ComponentOutcome::Changed
    }
    fn new(prt: &impl CompositeExt, sender: Sender<Self::Event>) -> Self {
        let group = foxtk::GroupBox::new(prt, "Selectors");
        let items = ["Item 1", "Item 2", "Item 3", "Item 4", "Item 5"];
        let listbox = foxtk::ListBox::new(&group)
            .with_items(&items)
            .with_num_visible(3)
            .with_callback({
                let sender = sender.clone();
                move |wgt| {
                    sender.send(Msg::Set(wgt.current_item())).unwrap();
                    false
                }
            });
        let list = foxtk::List::new(&group)
            .with_items(&items)
            .with_num_visible(3)
            .with_callback({
                let sender = sender.clone();
                move |wgt| {
                    if wgt.has_focus() {
                        sender.send(Msg::Set(wgt.current_item())).unwrap();
                    }
                    false
                }
            });
        Self { list, listbox }
    }
}
