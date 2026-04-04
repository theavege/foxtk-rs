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
struct Simple(
    foxtk::TextField,
    foxtk::Spinner,
    foxtk::RangeSlider,
    Option<foxtk::ComboBox>,
    Option<foxtk::ListBox>,
    Option<foxtk::Text>,
    Option<foxtk::TreeList>,
    Option<foxtk::Label>,
    Option<foxtk::Table>,
    Option<foxtk::Canvas>,
    Option<foxtk::TabBook>,
    Option<foxtk::ScrollBar>,
    Option<foxtk::MenuBar>,
);

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
        self.3 = Some(foxtk::ComboBox::new(&hbox, 10));
        if let Some(ref combo) = self.3 {
            combo.append_item("Option 1");
            combo.append_item("Option 2");
            combo.append_item("Option 3");
            combo.set_callback({
                let _sender = sender.clone();
                move |_combo: foxtk::ComboBox| {
                    // Maybe do something
                    false
                }
            });
        }
        self.4 = Some(foxtk::ListBox::new(&hbox));
        if let Some(ref list) = self.4 {
            list.append_item("Item 1");
            list.append_item("Item 2");
            list.append_item("Item 3");
            list.set_callback({
                let _sender = sender.clone();
                move |_list: foxtk::ListBox| {
                    // Maybe do something
                    false
                }
            });
        }
        self.5 = Some(foxtk::Text::new(&vbox));
        if let Some(ref text) = self.5 {
            text.set_text("This is a multi-line text editor.\nYou can edit this text.");
        }
        self.6 = Some(foxtk::TreeList::new(&vbox));
        if let Some(ref tree) = self.6 {
            let root = tree.add_item_first(None, "Root");
            tree.add_item_first(Some(&root), "Child 1");
            tree.add_item_first(Some(&root), "Child 2");
        }
        self.7 = Some(foxtk::Label::new(&vbox, "This is a label"));
        self.8 = Some(foxtk::Table::new(&vbox));
        if let Some(ref table) = self.8 {
            table.set_table_size(3, 3);
            table.set_item_text(0, 0, "A1");
            table.set_item_text(0, 1, "B1");
            table.set_item_text(1, 0, "A2");
        }
        self.9 = Some(foxtk::Canvas::new(&vbox));
        self.10 = Some(foxtk::TabBook::new(&vbox));
        if let Some(ref tabbook) = self.10 {
            let _tab1 = foxtk::TabItem::new(tabbook, "Tab 1");
            let _tab2 = foxtk::TabItem::new(tabbook, "Tab 2");
        }
        self.11 = Some(foxtk::ScrollBar::new(&vbox));
        if let Some(ref scrollbar) = self.11 {
            scrollbar.set_range(0, 100);
            scrollbar.set_position(50);
        }
        self.12 = Some(foxtk::MenuBar::new(&vbox));
        if let Some(ref menubar) = self.12 {
            let pane = foxtk::MenuPane::new(menubar);
            let _title = foxtk::MenuTitle::new(menubar, "File", &pane);
            let _cmd = foxtk::MenuCommand::new(&pane, "Open");
        }
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
