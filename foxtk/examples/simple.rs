#![forbid(unsafe_code)]

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
    fn view(&mut self, prt: &impl CompositeExt, sender: Sender<Self::Event>) {
        foxtk::VerticalFrame::new(prt)
            .with_frame(Frame::Line)
            .with_layout(Layout::Fill)
            .inside(|prt| {
                foxtk::MenuBar::new(prt)
                    .with_frame(Frame::Line)
                    .with_layout(Layout::FillX)
                    .inside(|prt| {
                        foxtk::MenuPane::new(prt).inside(|mpaine| {
                            foxtk::MenuTitle::new(prt, "Nav", mpaine);
                            foxtk::MenuCommand::new(mpaine, "Converter").set_callback({
                                let sender = sender.clone();
                                move |_| {
                                    sender.send(Msg::SetVal(0)).unwrap();
                                    false
                                }
                            });
                            foxtk::MenuCommand::new(mpaine, "Calc").set_callback({
                                let sender = sender.clone();
                                move |_| {
                                    sender.send(Msg::SetVal(1)).unwrap();
                                    false
                                }
                            });
                            foxtk::MenuCommand::new(mpaine, "NicCalc").set_callback({
                                let sender = sender.clone();
                                move |_| {
                                    sender.send(Msg::SetVal(2)).unwrap();
                                    false
                                }
                            });
                        });
                    });
                self.0 = foxtk::Switcher::new(prt)
                    .with_layout(Layout::Fill)
                    .inside(|prt| {
                        foxtk::VerticalFrame::new(prt).inside(|prt| {
                            components::Converter::mount(prt);
                            components::Rangers::mount(prt);
                            components::Selectors::mount(prt);
                        });
                        components::Calc::mount(prt);
                        components::NicCalc::mount(prt);
                    });
            });
    }
}

fn main() {
    Simple::run("Name", "Vendor", "Title");
}
