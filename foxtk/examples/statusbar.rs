#![forbid(unsafe_code)]

use foxtk::prelude::*;

#[derive(Default)]
struct StatusBarDemo {
    statusbar: foxtk::StatusBar,
}

impl Component for StatusBarDemo {
    type Event = Msg;
    type State = i32;

    fn handle(msg: Self::Event, model: &mut Self::State, _sender: Sender<Self::Event>) -> bool {
        match msg {
            Msg::Increment => {
                *model += 1;
            }
            Msg::Decrement => {
                *model -= 1;
            }
            Msg::Reset => {
                *model = 0;
            }
        }
        true
    }

    fn update(&self, model: &Self::State) {
        self.statusbar
            .set_text(&format!("Counter: {}", model));
    }

    fn view(&mut self, parent: &impl CompositeExt, sender: Sender<Self::Event>) {
        foxtk::VerticalFrame::new(parent).inside(|prt| {
            foxtk::Label::new(prt, "StatusBar Demo")
                .with_font("Arial", 16)
                .with_layout(Layout::FillX);

            foxtk::GroupBox::new(prt, "Controls").inside(|prt| {
                foxtk::HorizontalFrame::new(prt).inside(|prt| {
                    foxtk::Button::new(prt, "Increment").with_callback({
                        let sender = sender.clone();
                        move |_| {
                            sender.send(Msg::Increment).unwrap();
                            false
                        }
                    });

                    foxtk::Button::new(prt, "Decrement").with_callback({
                        let sender = sender.clone();
                        move |_| {
                            sender.send(Msg::Decrement).unwrap();
                            false
                        }
                    });

                    foxtk::Button::new(prt, "Reset").with_callback({
                        let sender = sender.clone();
                        move |_| {
                            sender.send(Msg::Reset).unwrap();
                            false
                        }
                    });
                });
            });

            // Status bar at the bottom
            self.statusbar = foxtk::StatusBar::new(prt);
            self.statusbar.set_text("Ready");
            self.statusbar.set_help_text("Shows the current counter value");
        });
    }
}

enum Msg {
    Increment,
    Decrement,
    Reset,
}

fn main() {
    StatusBarDemo::run("StatusBar Demo", "FOX Toolkit", "StatusBar Example", 400, 300);
}
