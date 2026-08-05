#![forbid(unsafe_code)]

use foxtk::prelude::*;

#[derive(Default)]
struct TabBookDemo {
    tabbook: foxtk::TabBook,
    statusbar: foxtk::StatusBar,
}

impl Component for TabBookDemo {
    type Event = Msg;
    type State = i32;

    fn handle(msg: Self::Event, _model: &mut Self::State, _sender: Sender<Self::Event>) -> bool {
        match msg {
            Msg::SwitchToTab(_index) => {
                // This would switch tabs, but we need to handle it in the view
            }
            Msg::UpdateStatus(_text) => {
                // Status updates are handled in the view
            }
        }
        true
    }

    fn update(&self, model: &Self::State) {
        self.statusbar
            .set_text(&format!("TabBook Demo - Counter: {}", model));
    }

    fn view(&mut self, parent: &impl CompositeExt, _sender: Sender<Self::Event>) {
        foxtk::VerticalFrame::new(parent).inside(|prt| {
            foxtk::Label::new(prt, "TabBook Demo")
                .with_font("Arial", 16)
                .with_layout(Layout::FillX);

            // Create a TabBook
            self.tabbook = foxtk::TabBook::new(prt).with_layout(Layout::Fill);

            self.tabbook.inside(|prt| {
                // Tab 1 - Editor
                let tab1 = foxtk::TabItem::new(prt, "Editor");
                tab1.inside(|prt| {
                    foxtk::VerticalFrame::new(prt).inside(|prt| {
                        foxtk::Label::new(prt, "Text Editor Tab").with_layout(Layout::FillX);
                        foxtk::Text::new(prt)
                            .with_layout(Layout::Fill)
                            .set_text("Type your text here...\nLine 2\nLine 3");
                    });
                });

                // Tab 2 - Settings
                let tab2 = foxtk::TabItem::new(prt, "Settings");
                tab2.inside(|prt| {
                    foxtk::VerticalFrame::new(prt).inside(|prt| {
                        foxtk::Label::new(prt, "Settings Tab").with_layout(Layout::FillX);
                        foxtk::GroupBox::new(prt, "Display Settings").inside(|prt| {
                            foxtk::CheckButton::new(prt, "Show line numbers");
                            foxtk::CheckButton::new(prt, "Syntax highlighting");
                            foxtk::CheckButton::new(prt, "Auto indent");
                        });
                        foxtk::GroupBox::new(prt, "Editor Settings").inside(|prt| {
                            foxtk::CheckButton::new(prt, "Tab characters");
                            foxtk::CheckButton::new(prt, "Spaces instead of tabs");
                        });
                    });
                });

                // Tab 3 - About
                let tab3 = foxtk::TabItem::new(prt, "About");
                tab3.inside(|prt| {
                    foxtk::VerticalFrame::new(prt).inside(|prt| {
                        foxtk::Label::new(prt, "About TabBook Demo").with_font("Arial", 14).with_layout(Layout::FillX);
                        foxtk::Label::new(prt, "This is a demonstration of the TabBook widget.").with_layout(Layout::FillX);
                        foxtk::Label::new(prt, "").with_layout(Layout::FillX);
                        foxtk::Label::new(prt, "Features:").with_layout(Layout::FillX);
                        foxtk::Label::new(prt, "- Multiple tabs with different content").with_layout(Layout::FillX);
                        foxtk::Label::new(prt, "- Easy navigation between tabs").with_layout(Layout::FillX);
                        foxtk::Label::new(prt, "- Customizable tab appearance").with_layout(Layout::FillX);
                    });
                });
            });

            // Navigation buttons
            foxtk::HorizontalFrame::new(prt).inside(|prt| {
                foxtk::Button::new(prt, "Tab 1").with_callback({
                    let tabbook = self.tabbook;
                    move |_| {
                        tabbook.set_current(0);
                        false
                    }
                });

                foxtk::Button::new(prt, "Tab 2").with_callback({
                    let tabbook = self.tabbook;
                    move |_| {
                        tabbook.set_current(1);
                        false
                    }
                });

                foxtk::Button::new(prt, "Tab 3").with_callback({
                    let tabbook = self.tabbook;
                    move |_| {
                        tabbook.set_current(2);
                        false
                    }
                });
            });

            // Status bar at the bottom
            self.statusbar = foxtk::StatusBar::new(prt);
            self.statusbar.set_text("Select a tab to see different content");
        });
    }
}

#[allow(dead_code)]
enum Msg {
    SwitchToTab(usize),
    UpdateStatus(String),
}

fn main() {
    TabBookDemo::run("TabBook Demo", "FOX Toolkit", "TabBook Example", 600, 500);
}
