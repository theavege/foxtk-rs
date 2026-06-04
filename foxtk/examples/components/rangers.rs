use foxtk::prelude::*;

pub enum Msg {
    Set(i32),
    Add(i32),
}

#[derive(Default)]
pub struct Rangers {
    progress: foxtk::ProgressBar,
    slider: foxtk::Slider,
    spinner: foxtk::Spinner,
    label: foxtk::Label,
}

impl Component for Rangers {
    type Event = Msg;
    type State = i32;
    fn handle(msg: Self::Event, model: &mut Self::State, _: Sender<Self::Event>) -> bool {
        match msg {
            Msg::Set(value) => *model = value,
            Msg::Add(value) => *model += value,
        };
        true
    }
    fn update(&self, model: &Self::State) {
        self.progress.set_value(*model as u32);
        self.label.set_text(&model.to_string());
        self.spinner.update(*model);
        self.slider.update(*model);
    }
    fn view(&mut self, prt: &impl CompositeExt, sender: Sender<Self::Event>) {
        foxtk::GroupBox::new(prt, "Ranges")
            .with_frame(FrameStyle::Line)
            .with_layout(Layout::FillX)
            .inside(|prt| {
                foxtk::VerticalFrame::new(prt)
                    .with_frame(FrameStyle::Line)
                    .with_layout(Layout::FillX)
                    .inside(|prt| {
                        foxtk::HorizontalFrame::new(prt).inside(|prt| {
                            foxtk::Button::new(prt, "Prev").set_callback({
                                let sender = sender.clone();
                                move |wgt| {
                                    if wgt.has_focus() {
                                        sender.send(Msg::Add(-1)).unwrap();
                                    }
                                    false
                                }
                            });
                            self.label = foxtk::Label::new(prt, "");
                            foxtk::Button::new(prt, "Next")
                                .with_tip("This is the tooltip for the button.")
                                .set_callback({
                                    let sender = sender.clone();
                                    move |wgt| {
                                        if wgt.has_focus() {
                                            sender.send(Msg::Add(1)).unwrap();
                                        }
                                        false
                                    }
                                });
                            foxtk::ArrowButton::new(prt)
                                .with_callback(|wgt| {
                                    wgt.message(MessageBox::Ok, "CANCEL", Message::Error);
                                    false
                                })
                                .set_color(Color::from_rgb(220, 50, 47));
                            foxtk::TreeList::new(prt);
                            foxtk::RadioButton::new(prt, "Radio");
                            foxtk::CheckButton::new(prt, "Check");
                            foxtk::ToggleButton::new(prt, "Toggle", "Toggle_");
                            foxtk::Knob::new(prt);
                        });
                        self.spinner = foxtk::Spinner::new(prt)
                            .with_range(0, 8)
                            .with_increment(1)
                            .with_callback({
                                let sender = sender.clone();
                                move |wgt| {
                                    if wgt.has_focus() {
                                        sender.send(Msg::Set(wgt.value())).unwrap();
                                    }
                                    false
                                }
                            });
                        self.progress = foxtk::ProgressBar::new(prt).with_total(8);
                        self.slider = foxtk::Slider::new(prt)
                            .with_range(0, 8)
                            .with_increment(1)
                            .with_callback({
                                let sender = sender.clone();
                                move |wgt| {
                                    if wgt.has_focus() {
                                        sender.send(Msg::Set(wgt.value())).unwrap();
                                    }
                                    false
                                }
                            });
                    });
            });
    }
}
