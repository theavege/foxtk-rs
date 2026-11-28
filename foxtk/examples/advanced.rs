#![forbid(unsafe_code)]

use foxtk::prelude::*;
mod components;

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
    fn new(prt: &impl CompositeExt, sender: Sender<Self::Event>) -> Self {
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
        let switcher = foxtk::Switcher::new(prt);
        let mut mounts = Vec::new();
        let first = foxtk::VerticalFrame::new(&switcher);
        mounts.push(components::Rangers::mount(&first));
        mounts.push(components::Converter::mount(&first));
        mounts.push(components::Booker::mount(&first));
        mounts.push(components::Selectors::mount(&first));
        mounts.push(components::Timer::mount(&switcher));
        mounts.push(components::Crud::mount(&switcher));
        mounts.push(components::Circles::mount(&switcher));
        mounts.push(components::Cells::mount(&switcher));
        Self { switcher, mounts }
    }
}

fn main() {
    Example::run("Name", "Vendor", "Advanced Example", 400, 640);
}
