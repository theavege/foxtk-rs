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
        self.spinner.set_value(*model);
        self.slider.set_value(*model);
        self.label.set_text(&model.to_string());
    }
    fn view(&mut self, prt: &impl CompositeExt, sender: Sender<Self::Event>) {
        foxtk::GroupBox::new(prt, "Ranges").inside(|prt| {
            foxtk::VerticalFrame::new(prt).inside(|prt| {
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
                    self.label = foxtk::Label::new(prt, "").with_width(8);
                    foxtk::Button::new(prt, "Next").set_callback({
                        let sender = sender.clone();
                        move |wgt| {
                            if wgt.has_focus() {
                                sender.send(Msg::Add(1)).unwrap();
                            }
                            false
                        }
                    });
                });
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
            self.progress = foxtk::ProgressBar::new(prt).with_total(8).with_width(6);
            self.slider = foxtk::Slider::new(prt)
                .with_trigger(Trigger::CHANGED)
                .with_width(6)
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
    }
}
