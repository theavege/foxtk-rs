#![forbid(unsafe_code)]

use foxtk::prelude::*;

#[allow(dead_code)]
#[derive(Default)]
struct WidgetsDemo {
    statusbar: foxtk::StatusBar,
    dialog_box: Option<foxtk::DialogBox>,
    file_dialog: Option<foxtk::FileDialog>,
    splitter: foxtk::Splitter,
    tabbook: foxtk::TabBook,
}

impl Component for WidgetsDemo {
    type Event = Msg;
    type State = i32;

    fn handle(msg: Self::Event, _model: &mut Self::State, _sender: Sender<Self::Event>) -> bool {
        match msg {
            Msg::ShowDialog => {
                // This would normally show a dialog, but we can't do it in this example
                // without unsafe code
            }
            Msg::ShowFileDialog => {
                // This would normally show a file dialog
            }
            Msg::UpdateStatus(_text) => {
                // Status updates are handled in the view
            }
        }
        true
    }

    fn update(&self, model: &Self::State) {
        self.statusbar
            .set_text(&format!("Status: Counter = {}", model));
    }

    fn view(&mut self, parent: &impl CompositeExt, sender: Sender<Self::Event>) {
        // Create a vertical frame as the main container
        foxtk::VerticalFrame::new(parent).inside(|prt| {
            // Header
            foxtk::Label::new(prt, "FOX Toolkit Widget Demo")
                .with_font("Arial", 16)
                .with_layout(Layout::FillX);

            // Splitter demonstration
            foxtk::Label::new(prt, "Splitter Widget:").with_layout(Layout::FillX);
            self.splitter = foxtk::Splitter::new(prt)
                .with_layout(Layout::Fill)
                .with_style(SplitterStyle::Horizontal);

            self.splitter.inside(|prt| {
                // Left pane
                foxtk::GroupBox::new(prt, "Left Pane").inside(|prt| {
                    foxtk::Label::new(prt, "Left Content").with_layout(Layout::FillX);
                    foxtk::Button::new(prt, "Button 1").with_callback({
                        let sender = sender.clone();
                        move |_| {
                            sender
                                .send(Msg::UpdateStatus("Button 1 clicked".to_string()))
                                .unwrap();
                            false
                        }
                    });
                });

                // Right pane
                foxtk::GroupBox::new(prt, "Right Pane").inside(|prt| {
                    foxtk::Label::new(prt, "Right Content").with_layout(Layout::FillX);
                    foxtk::Button::new(prt, "Button 2").with_callback({
                        let sender = sender.clone();
                        move |_| {
                            sender
                                .send(Msg::UpdateStatus("Button 2 clicked".to_string()))
                                .unwrap();
                            false
                        }
                    });
                });
            });

            // Set initial splitter position
            self.splitter.set_split(0, 200);

            // TabBook demonstration
            foxtk::Label::new(prt, "TabBook Widget:").with_layout(Layout::FillX);
            self.tabbook = foxtk::TabBook::new(prt).with_layout(Layout::Fill);

            self.tabbook.inside(|prt| {
                // Tab 1
                let tab1 = foxtk::TabItem::new(prt, "Tab 1");
                tab1.inside(|prt| {
                    foxtk::VerticalFrame::new(prt).inside(|prt| {
                        foxtk::Label::new(prt, "This is Tab 1").with_layout(Layout::FillX);
                        foxtk::Button::new(prt, "Tab 1 Button").with_callback({
                            let sender = sender.clone();
                            move |_| {
                                sender
                                    .send(Msg::UpdateStatus("Tab 1 button clicked".to_string()))
                                    .unwrap();
                                false
                            }
                        });
                    });
                });

                // Tab 2
                let tab2 = foxtk::TabItem::new(prt, "Tab 2");
                tab2.inside(|prt| {
                    foxtk::VerticalFrame::new(prt).inside(|prt| {
                        foxtk::Label::new(prt, "This is Tab 2").with_layout(Layout::FillX);
                        foxtk::Button::new(prt, "Tab 2 Button").with_callback({
                            let sender = sender.clone();
                            move |_| {
                                sender
                                    .send(Msg::UpdateStatus("Tab 2 button clicked".to_string()))
                                    .unwrap();
                                false
                            }
                        });
                    });
                });

                // Tab 3
                let tab3 = foxtk::TabItem::new(prt, "Tab 3");
                tab3.inside(|prt| {
                    foxtk::VerticalFrame::new(prt).inside(|prt| {
                        foxtk::Label::new(prt, "This is Tab 3").with_layout(Layout::FillX);
                        foxtk::Button::new(prt, "Tab 3 Button").with_callback({
                            let sender = sender.clone();
                            move |_| {
                                sender
                                    .send(Msg::UpdateStatus("Tab 3 button clicked".to_string()))
                                    .unwrap();
                                false
                            }
                        });
                    });
                });
            });

            // Buttons to show dialogs
            foxtk::HorizontalFrame::new(prt).inside(|prt| {
                foxtk::Button::new(prt, "Show DialogBox").with_callback({
                    let sender = sender.clone();
                    move |_| {
                        sender.send(Msg::ShowDialog).unwrap();
                        false
                    }
                });

                foxtk::Button::new(prt, "Show FileDialog").with_callback({
                    let sender = sender.clone();
                    move |_| {
                        sender.send(Msg::ShowFileDialog).unwrap();
                        false
                    }
                });
            });

            // Status bar at the bottom
            self.statusbar = foxtk::StatusBar::new(prt);
            self.statusbar.set_text("Ready");
            self.statusbar.set_help_text("This is a status bar demo");
        });
    }
}

enum Msg {
    ShowDialog,
    ShowFileDialog,
    UpdateStatus(String),
}

fn main() {
    WidgetsDemo::run("Widget Demo", "FOX Toolkit", "FOX Widget Demo", 800, 600);
}
