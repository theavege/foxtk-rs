#![forbid(unsafe_code)]

use foxtk::prelude::*;

#[derive(Default)]
struct DialogBoxDemo {
    statusbar: foxtk::StatusBar,
}

impl Component for DialogBoxDemo {
    type Event = Msg;
    type State = i32;

    fn handle(msg: Self::Event, _model: &mut Self::State, _sender: Sender<Self::Event>) -> bool {
        match msg {
            Msg::ShowDialog => {
                // In a real application, you would create and show a DialogBox here
                // For example:
                // let dialog = foxtk::DialogBox::new(&window, "My Dialog");
                // dialog.show();
            }
            Msg::UpdateStatus(text) => {
                // Status updates are handled in the view
            }
        }
        true
    }

    fn update(&self, model: &Self::State) {
        self.statusbar
            .set_text(&format!("DialogBox Demo - Counter: {}", model));
    }

    fn view(&mut self, parent: &impl CompositeExt, sender: Sender<Self::Event>) {
        foxtk::VerticalFrame::new(parent).inside(|prt| {
            foxtk::Label::new(prt, "DialogBox Demo")
                .with_font("Arial", 16)
                .with_layout(Layout::FillX);

            foxtk::GroupBox::new(prt, "DialogBox").inside(|prt| {
                foxtk::Label::new(prt, "Click the button below to show a dialog box")
                    .with_layout(Layout::FillX);

                foxtk::Button::new(prt, "Show DialogBox").with_callback({
                    let sender = sender.clone();
                    move |_| {
                        sender.send(Msg::ShowDialog).unwrap();
                        false
                    }
                });

                foxtk::Label::new(prt, "Note: DialogBox requires unsafe code to show properly")
                    .with_layout(Layout::FillX);
            });

            // Status bar at the bottom
            self.statusbar = foxtk::StatusBar::new(prt);
            self.statusbar.set_text("Ready");
        });
    }
}

enum Msg {
    ShowDialog,
    UpdateStatus(String),
}

fn main() {
    DialogBoxDemo::run("DialogBox Demo", "FOX Toolkit", "DialogBox Example", 400, 300);
}
