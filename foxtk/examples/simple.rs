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
            .with_frame(FrameStyle::Line)
            .with_layout(Layout::Fill)
            .inside(|prt| {
                foxtk::MenuBar::new(prt).inside(|prt| {
                    foxtk::MenuPane::new(prt).inside(|mpaine| {
                        foxtk::MenuTitle::new(prt, "Nav", mpaine);
                        for (idx, item) in ["Converter", "Calc", "NicCalc", "Dialect"]
                            .iter()
                            .enumerate()
                        {
                            foxtk::MenuCommand::new(mpaine, item).set_callback({
                                let sender = sender.clone();
                                move |_| {
                                    sender.send(Msg::SetVal(idx as i32)).unwrap();
                                    false
                                }
                            });
                        }
                    });
                    foxtk::MenuPane::new(prt).inside(|mpaine| {
                        foxtk::MenuTitle::new(prt, "View", mpaine);
                        foxtk::MenuCheck::new(mpaine, "Hidden files");
                        foxtk::MenuCheck::new(mpaine, "File Browser");
                        foxtk::MenuCheck::new(mpaine, "Toolbar");
                        foxtk::MenuCheck::new(mpaine, "Status line");
                        foxtk::MenuCheck::new(mpaine, "Undo Counters");
                        foxtk::MenuCheck::new(mpaine, "Clock");
                    });
                    foxtk::MenuPane::new(prt).inside(|mpaine| {
                        foxtk::MenuTitle::new(prt, "Window", mpaine);
                        foxtk::MenuRadio::new(mpaine, "1 untitled");
                        foxtk::MenuRadio::new(mpaine, "2 untitled");
                        foxtk::MenuRadio::new(mpaine, "3 untitled");
                    });
                });
                self.0 = foxtk::Switcher::new(prt)
                    .with_layout(Layout::Fill)
                    .inside(|prt| {
                        foxtk::VerticalFrame::new(prt).inside(|prt| {
                            components::Converter::mount(prt);
                            components::Rangers::mount(prt);
                            components::Selectors::mount(prt);
                            components::Nmap::mount(prt);
                            components::Booker::mount(prt);
                        });
                        components::Calc::mount(prt);
                        components::NicCalc::mount(prt);
                        components::Dialect::mount(prt);
                    });
            });
    }
}

fn main() {
    Simple::run("Name", "Vendor", "Title");
}
