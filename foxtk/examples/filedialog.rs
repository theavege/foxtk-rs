#![forbid(unsafe_code)]

use foxtk::prelude::*;

#[derive(Default)]
struct FileDialogDemo {
    statusbar: foxtk::StatusBar,
    filename_label: foxtk::Label,
}

impl Component for FileDialogDemo {
    type Event = Msg;
    type State = String;

    fn handle(msg: Self::Event, model: &mut Self::State, _sender: Sender<Self::Event>) -> bool {
        match msg {
            Msg::OpenFile => {
                // In a real application, you would use the WindowExt methods
                // to show a file dialog. The current implementation in prelude.rs
                // already has open_file_dialog and save_file_dialog methods.
                // For example:
                // let filename = window.open_file_dialog("Open File", ".", "*.txt", 0);
                // *model = filename;
            }
            Msg::SaveFile => {
                // Similar to OpenFile but for saving
            }
            Msg::UpdateFilename(filename) => {
                *model = filename;
            }
        }
        true
    }

    fn update(&self, model: &Self::State) {
        self.statusbar.set_text(&format!("Current file: {}", model));
        self.filename_label
            .set_text(&format!("Selected: {}", model));
    }

    fn view(&mut self, parent: &impl CompositeExt, sender: Sender<Self::Event>) {
        foxtk::VerticalFrame::new(parent).inside(|prt| {
            foxtk::Label::new(prt, "FileDialog Demo")
                .with_font("Arial", 16)
                .with_layout(Layout::FillX);

            foxtk::GroupBox::new(prt, "File Operations").inside(|prt| {
                foxtk::VerticalFrame::new(prt).inside(|prt| {
                    foxtk::Label::new(prt, "Click buttons to open file dialogs:")
                        .with_layout(Layout::FillX);

                    foxtk::Button::new(prt, "Open File...").with_callback({
                        let sender = sender.clone();
                        move |_| {
                            sender.send(Msg::OpenFile).unwrap();
                            false
                        }
                    });

                    foxtk::Button::new(prt, "Save File As...").with_callback({
                        let sender = sender.clone();
                        move |_| {
                            sender.send(Msg::SaveFile).unwrap();
                            false
                        }
                    });

                    foxtk::Label::new(prt, "").with_layout(Layout::FillX);
                    self.filename_label = foxtk::Label::new(prt, "No file selected");
                });
            });

            foxtk::GroupBox::new(prt, "Note").inside(|prt| {
                foxtk::Label::new(prt, "FileDialog requires unsafe code to show properly.")
                    .with_layout(Layout::FillX);
                foxtk::Label::new(
                    prt,
                    "The existing WindowExt methods (open_file_dialog, save_file_dialog)",
                )
                .with_layout(Layout::FillX);
                foxtk::Label::new(prt, "can be used in safe code through the prelude.")
                    .with_layout(Layout::FillX);
            });

            // Status bar at the bottom
            self.statusbar = foxtk::StatusBar::new(prt);
            self.statusbar.set_text("Ready");
        });
    }
}

#[allow(dead_code)]
enum Msg {
    OpenFile,
    SaveFile,
    UpdateFilename(String),
}

fn main() {
    FileDialogDemo::run(
        "FileDialog Demo",
        "FOX Toolkit",
        "FileDialog Example",
        500,
        400,
    );
}
