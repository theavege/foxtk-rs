#![forbid(unsafe_code)]

use foxtk::prelude::*;

#[derive(Default)]
struct SplitterDemo {
    splitter: foxtk::Splitter,
    statusbar: foxtk::StatusBar,
}

impl Component for SplitterDemo {
    type Event = Msg;
    type State = i32;

    fn handle(msg: Self::Event, _model: &mut Self::State, _sender: Sender<Self::Event>) -> bool {
        match msg {
            Msg::UpdateStatus(_text) => {
                // Status updates are handled in the view
            }
        }
        true
    }

    fn update(&self, model: &Self::State) {
        self.statusbar
            .set_text(&format!("Splitter Demo - Counter: {}", model));
    }

    fn view(&mut self, parent: &impl CompositeExt, _sender: Sender<Self::Event>) {
        foxtk::VerticalFrame::new(parent).inside(|prt| {
            foxtk::Label::new(prt, "Splitter Demo")
                .with_font("Arial", 16)
                .with_layout(Layout::FillX);

            // Create a splitter with horizontal orientation
            self.splitter = foxtk::Splitter::new(prt)
                .with_layout(Layout::Fill)
                .with_style(SplitterStyle::Horizontal);

            self.splitter.inside(|prt| {
                // Left pane - a text editor
                foxtk::GroupBox::new(prt, "Left Pane - Text Editor").inside(|prt| {
                    foxtk::Text::new(prt)
                        .with_layout(Layout::Fill)
                        .set_text("This is the left pane.\nYou can type text here.\nThe splitter allows you to resize this pane.");
                });

                // Right pane - controls
                foxtk::GroupBox::new(prt, "Right Pane - Controls").inside(|prt| {
                    foxtk::VerticalFrame::new(prt).inside(|prt| {
                        foxtk::Label::new(prt, "Splitter Controls").with_layout(Layout::FillX);

                        foxtk::Button::new(prt, "Set Split to 50%").with_callback({
                            let splitter = self.splitter;
                            move |_| {
                                // Get the total width and set split to 50%
                                // In a real app, you would calculate this based on actual sizes
                                splitter.set_split(0, 300);
                                false
                            }
                        });

                        foxtk::Button::new(prt, "Set Bar Size").with_callback({
                            let splitter = self.splitter;
                            move |_| {
                                splitter.set_bar_size(10);
                                false
                            }
                        });

                        foxtk::Label::new(prt, &format!("Current bar size: {}", self.splitter.bar_size()))
                            .with_layout(Layout::FillX);
                    });
                });
            });

            // Set initial splitter position
            self.splitter.set_split(0, 300);

            // Status bar at the bottom
            self.statusbar = foxtk::StatusBar::new(prt);
            self.statusbar.set_text("Drag the splitter bar to resize panes");
        });
    }
}

#[allow(dead_code)]
enum Msg {
    UpdateStatus(String),
}

fn main() {
    SplitterDemo::run("Splitter Demo", "FOX Toolkit", "Splitter Example", 800, 600);
}
