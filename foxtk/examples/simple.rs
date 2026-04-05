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
    (),
    Option<foxtk::Table>,
    (),
    Option<foxtk::TabBook>,
    (),
    Option<foxtk::MenuBar>,
);

impl Component for Simple {
    type Event = Msg;
    type State = models::Model;
    fn handle(msg: Self::Event, model: &mut Self::State, _: Sender<Self::Event>) -> bool {
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
    fn view(&mut self, parent: &impl WindowExt, sender: Sender<Self::Event>) {
        foxtk::VerticalFrame::new(parent).inside(|vbox| {
            self.12 = Some(foxtk::MenuBar::new(vbox));
            if let Some(ref menubar) = self.12 {
                let pane = foxtk::MenuPane::new(menubar);
                let _title = foxtk::MenuTitle::new(menubar, "File", &pane);
                let _cmd = foxtk::MenuCommand::new(&pane, "Open");
            }
            foxtk::HorizontalFrame::new(vbox).inside(|hbox| {
                foxtk::Button::new(hbox, "plus").set_callback({
                    let sender = sender.clone();
                    move |wgt| {
                        sender.send(Msg::SetVal(1)).unwrap();
                        println!("{}", wgt.text());
                        false
                    }
                });
                self.0 = foxtk::TextField::new(hbox, 6);
                foxtk::Button::new(hbox, "minus").set_callback({
                    let sender = sender.clone();
                    move |wgt| {
                        println!("{}", wgt.text());
                        sender.send(Msg::SetVal(-1)).unwrap();
                        false
                    }
                });
                self.1 = foxtk::Spinner::new(hbox, 6)
                    .with_range(0, 8)
                    .with_increment(1)
                    .with_callback({
                        let sender = sender.clone();
                        move |spinner: foxtk::Spinner| {
                            sender.send(Msg::SetValue(spinner.value())).unwrap();
                            false
                        }
                    });
            });
            components::Converter::mount(vbox);
            foxtk::HorizontalFrame::new(vbox).inside(|hbox| {
                components::RadioExample::mount(hbox);
                components::CheckExample::mount(hbox);
                self.5 = Some(foxtk::Text::new(hbox));
                if let Some(ref text) = self.5 {
                    text.set_text("This is a multi-line text editor.\nYou can edit this text.");
                }
                self.6 = Some(foxtk::TreeList::new(hbox));
                if let Some(ref tree) = self.6 {
                    let root = tree.add_item_first(None, "Root");
                    tree.add_item_first(Some(&root), "Child 1");
                    tree.add_item_first(Some(&root), "Child 2");
                }
                self.8 = Some(foxtk::Table::new(hbox));
                if let Some(ref table) = self.8 {
                    table.set_table_size(3, 3);
                    table.set_item_text(0, 0, "A1");
                    table.set_item_text(0, 1, "B1");
                    table.set_item_text(1, 0, "A2");
                }
            });
            foxtk::HorizontalFrame::new(vbox).inside(|hbox| {
                self.10 = Some(foxtk::TabBook::new(hbox));
                if let Some(ref tabbook) = self.10 {
                    let _tab1 = foxtk::TabItem::new(tabbook, "Tab 1");
                    let _tab2 = foxtk::TabItem::new(tabbook, "Tab 2");
                }
            });
            self.2 = foxtk::RangeSlider::new(vbox);
            self.2.set_range(0, 100);
            self.2.set_increment(1);
            self.2.set_callback({
                let sender = sender.clone();
                move |slider: foxtk::RangeSlider| {
                    sender.send(Msg::SetValue(slider.value())).unwrap();
                    false
                }
            });
            let hbox = foxtk::HorizontalFrame::new(vbox);
            self.3 = Some(foxtk::ComboBox::new(&hbox, 10));
            if let Some(ref combo) = self.3 {
                combo.append_item("Option 1");
                combo.append_item("Option 2");
                combo.append_item("Option 3");
                combo.set_callback({
                    move |wgt| {
                        println!(
                            "{}:{}",
                            wgt.current_item(),
                            wgt.item_text(wgt.current_item())
                        );
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
                    move |wgt| {
                        println!(
                            "{}:{}",
                            wgt.current_item(),
                            wgt.item_text(wgt.current_item())
                        );
                        false
                    }
                });
            }
        });
    }
}

fn main() {
    Simple::run("Name", "Vendor", "Title");
}
