use foxtk::prelude::*;

pub enum Msg {
    Set(i32),
}

#[derive(Default)]
pub struct Rangers {
    progress: foxtk::ProgressBar,
    slider: foxtk::Slider,
    spinner: foxtk::Spinner,
}

impl Component for Rangers {
    type Event = Msg;
    type State = i32;
    fn handle(msg: Self::Event, model: &mut Self::State, _: Sender<Self::Event>) -> bool {
        match msg {
            Msg::Set(value) => *model = value,
        };
        true
    }
    fn update(&self, model: &Self::State) {
        self.progress.set_value(*model as u32);
        //~ self.spinner.update(*model);
        self.slider.update(*model);
    }
    fn view(&mut self, prt: &impl CompositeExt, sender: Sender<Self::Event>) {
        const WIDTH: i32 = 90;
        foxtk::GroupBox::new(prt, "Rangers").inside(|prt| {
            foxtk::VerticalFrame::new(prt).inside(|prt| {
                foxtk::HorizontalFrame::new(prt).inside(|prt| {
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
                });
                foxtk::HorizontalFrame::new(prt).inside(|prt| {
                    self.progress = foxtk::ProgressBar::new(prt).with_total(8);
                    self.slider = foxtk::Slider::new(prt)
                        .with_width(WIDTH)
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
        });
    }
}
