pub mod models {
    #[derive(Default)]
    pub struct Model(bool, String);

    impl Model {
        pub fn changed(&self) -> bool {
            self.0
        }
        pub fn set_changed(&mut self, changed: bool) {
            self.0 = changed;
        }
        pub fn set_content(&mut self, content: String) {
            self.1 = content;
        }
        pub fn content(&self) -> &String {
            &self.1
        }
    }
}

use foxtk::prelude::*;

pub enum Msg {
    SetContent(String),
    Open(String),
    Save(String),
}

#[derive(Default)]
pub struct Adie {
    content: foxtk::Text,
    save: foxtk::Button,
}

impl Component for Adie {
    type Event = Msg;
    type State = models::Model;

    fn handle(msg: Self::Event, model: &mut Self::State, _: Sender<Self::Event>) -> bool {
        match msg {
            Msg::Open(path) => model.set_content(std::fs::read_to_string(path).unwrap()),
            Msg::Save(path) => {
                std::fs::write(path, model.content().as_bytes()).unwrap();
                model.set_changed(false);
                return false;
            }
            Msg::SetContent(value) => {
                model.set_content(value);
                model.set_changed(true);
            }
        }
        true
    }

    fn update(&self, model: &Self::State) {
        if !self.content.has_focus() {
            self.content.set_text(model.content());
        }
        self.save.set_enable(model.changed());
    }

    fn view(&mut self, prt: &impl CompositeExt, sender: Sender<Self::Event>) {
        foxtk::VerticalFrame::new(prt)
            .with_layout(Layout::Fill)
            .with_frame(FrameStyle::Thick)
            .inside(|prt| {
                foxtk::HorizontalFrame::new(prt)
                    .with_layout(Layout::FillX)
                    .with_height(40)
                    .inside(|prt| {
                        foxtk::Button::new(prt, "Open").with_callback({
                            let sender = sender.clone();
                            move |wgt| {
                                let path = wgt.open_file_dialog(
                                    "Open File",
                                    &std::env::var(match cfg!(target_os = "windows") {
                                        true => "HOMEPATH",
                                        false => "HOME",
                                    })
                                    .unwrap(),
                                    "*.md",
                                    0,
                                );
                                if !path.is_empty() {
                                    sender.send(Msg::Open(path)).unwrap();
                                }
                                false
                            }
                        });
                        self.save = foxtk::Button::new(prt, "Save").with_callback({
                            let sender = sender.clone();
                            move |wgt| {
                                let path = wgt.save_file_dialog(
                                    "Save File",
                                    &std::env::var(match cfg!(target_os = "windows") {
                                        true => "HOMEPATH",
                                        false => "HOME",
                                    })
                                    .unwrap(),
                                    "*.md",
                                    0,
                                );
                                if !path.is_empty() {
                                    sender.send(Msg::Save(path)).unwrap();
                                }
                                false
                            }
                        });
                    });
                self.content = foxtk::Text::new(prt)
                    .with_layout(Layout::Fill)
                    .with_editable(true)
                    .with_font("helvetica", 14)
                    .with_callback({
                        let sender = sender.clone();
                        move |wgt| {
                            if wgt.has_focus() {
                                sender.send(Msg::SetContent(wgt.text())).unwrap();
                            }
                            false
                        }
                    });
            });
    }
}
