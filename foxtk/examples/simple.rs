enum Msg {
    SetVal(i32),
}

use foxtk::prelude::*;
mod components;

#[derive(Default)]
struct Simple(foxtk::Switcher);

impl Component for Simple {
    type Event = Msg;
    type State = i32;
    fn handle(msg: Self::Event, model: &mut Self::State, _: Sender<Self::Event>) -> bool {
        match msg {
            Msg::SetVal(val) => *model = val,
        };
        true
    }
    fn update(&self, model: &Self::State) {
        self.0.set_curent(*model);
    }
    fn view(&mut self, parent: &impl CompositeExt, sender: Sender<Self::Event>) {
        foxtk::VerticalFrame::new(parent).inside(|vbox| {
            foxtk::MenuBar::new(vbox).inside(|mbar|{
                foxtk::MenuPane::new(mbar).inside(|mpaine| {
                    foxtk::MenuTitle::new(mbar, "Nav", mpaine);
                    foxtk::MenuCommand::new(mpaine, "Converter").set_callback({
                        let sender = sender.clone();
                        move|_| {
                            sender.send(Msg::SetVal(0)).unwrap();
                            false
                        }
                    });
                    foxtk::MenuCommand::new(mpaine, "Rangers").set_callback({
                        let sender = sender.clone();
                        move|_| {
                            sender.send(Msg::SetVal(1)).unwrap();
                            false
                        }
                    });
                    foxtk::MenuCommand::new(mpaine, "Selectors").set_callback({
                        let sender = sender.clone();
                        move|_| {
                            sender.send(Msg::SetVal(2)).unwrap();
                            false
                        }
                    });
                });
            });
        });
        self.0 = foxtk::Switcher::new(parent).inside(|swt| {
            components::Converter::mount(swt);
            components::Rangers::mount(swt);
            components::Selectors::mount(swt);
        });
    }
}

fn main() {
    Simple::run("Name", "Vendor", "Title");
}
