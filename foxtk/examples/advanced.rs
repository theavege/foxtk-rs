#![forbid(unsafe_code)]

enum Msg {
    SetVal(i32),
}

use foxtk::prelude::*;
mod components;

#[derive(Default)]
struct Simple {
    switcher: foxtk::Switcher,
    mounts: Vec<ComponentMount>,
}

impl Component for Simple {
    type Event = Msg;
    type State = i32;
    fn handle(
        msg: Self::Event,
        model: &mut Self::State,
        _: Sender<Self::Event>,
    ) -> ComponentOutcome {
        match msg {
            Msg::SetVal(val) => *model = val,
        };
        ComponentOutcome::Changed
    }
    fn update(&self, model: &Self::State) {
        self.switcher.set_curent(*model);
    }
    fn view(&mut self, prt: &impl CompositeExt, sender: Sender<Self::Event>) {
        foxtk::MenuBar::new(prt).inside(|prt| {
            foxtk::MenuPane::new(prt).inside(|mpaine| {
                foxtk::MenuTitle::new(prt, "Nav", mpaine);
                for (idx, item) in [
                    "Widgets",
                    "FOX Calculator",
                    "Adie",
                    "Timer",
                    "Dialect",
                    "NicCalc",
                ]
                .iter()
                .enumerate()
                {
                    foxtk::MenuCommand::new(mpaine, item).with_callback({
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
        self.switcher = foxtk::Switcher::new(prt);
        let mut mounts = Vec::new();
        self.switcher.inside(|prt| {
            foxtk::VerticalFrame::new(prt).inside(|prt| {
                foxtk::TabBook::new(prt).inside(|prt| {
                    foxtk::TabItem::new(prt, "Tab 1");
                    foxtk::TabItem::new(prt, "Tab 2");
                    foxtk::TabItem::new(prt, "Tab 3");
                });
                foxtk::GroupBox::new(prt, "Inputs").inside(|prt| {
                    mounts.push(components::Converter::mount(prt));
                    mounts.push(components::Nmap::mount(prt));
                });
                mounts.push(components::Rangers::mount(prt));
                mounts.push(components::Selectors::mount(prt));
            });
            mounts.push(components::Calc::mount(prt));
            mounts.push(components::Adie::mount(prt));
            mounts.push(components::Timer::mount(prt));
            mounts.push(components::Dialect::mount(prt));
            mounts.push(components::NicCalc::mount(prt));
        });
        self.mounts = mounts;
    }
}

fn main() {
    Simple::run("Name", "Vendor", "Title", 640, 400);
}
