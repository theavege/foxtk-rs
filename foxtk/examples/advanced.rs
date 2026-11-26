#![forbid(unsafe_code)]

use foxtk::prelude::*;
mod components;

#[derive(Default)]
struct Example {
    switcher: foxtk::Switcher,
    mounts: Vec<ComponentMount>,
}

impl Component for Example {
    type Event = i32;
    type State = i32;
    fn handle(
        msg: Self::Event,
        model: &mut Self::State,
        _: Sender<Self::Event>,
    ) -> ComponentOutcome {
        *model = msg;
        ComponentOutcome::Changed
    }
    fn update(&self, model: &Self::State) {
        self.switcher.set_current(*model);
    }
    fn view(&mut self, prt: &impl CompositeExt, sender: Sender<Self::Event>) {
        foxtk::MenuBar::new(prt).inside(|prt| {
            foxtk::MenuPane::new(prt).inside(|mpaine| {
                foxtk::MenuTitle::new(prt, "7GUIs", mpaine);
                for (idx, item) in [
                    "Counter, Converter, Booker",
                    "Timer",
                    "CRUD",
                    "Circle",
                    "Cells",
                ]
                .iter()
                .enumerate()
                {
                    foxtk::MenuCommand::new(mpaine, item).with_callback({
                        let sender = sender.clone();
                        move |_| {
                            sender.send(idx as i32).unwrap();
                            false
                        }
                    });
                }
            });
            foxtk::MenuPane::new(prt).inside(|mpaine| {
                foxtk::MenuTitle::new(prt, "View", mpaine);
                foxtk::MenuCheck::new(mpaine, "Hidden files");
                foxtk::MenuCheck::new(mpaine, "File Browser");
            });
            foxtk::MenuPane::new(prt).inside(|mpaine| {
                foxtk::MenuTitle::new(prt, "Window", mpaine);
                foxtk::MenuRadio::new(mpaine, "1 untitled");
                foxtk::MenuRadio::new(mpaine, "2 untitled");
            });
        });
        self.switcher = foxtk::Switcher::new(prt);
        let mut mounts = Vec::new();
        self.switcher.inside(|prt| {
            foxtk::VerticalFrame::new(prt).inside(|prt| {
                mounts.push(components::Rangers::mount(prt));
                mounts.push(components::Converter::mount(prt));
                mounts.push(components::Booker::mount(prt));
                mounts.push(components::Selectors::mount(prt));
            });
            mounts.push(components::Timer::mount(prt));
            mounts.push(components::Crud::mount(prt));
            mounts.push(components::Circles::mount(prt));
            mounts.push(components::Cells::mount(prt));
        });
        self.mounts = mounts;
    }
}

fn main() {
    Example::run("Name", "Vendor", "Advanced Example", 400, 640);
}
