use foxtk::prelude::*;

pub enum Msg {
    Set(i32),
}

pub struct Rangers {
    progress: foxtk::ProgressBar,
    slider: foxtk::Slider,
    spinner: foxtk::Spinner,
}

impl Component for Rangers {
    type Event = Msg;
    type State = i32;
    fn handle(
        msg: Self::Event,
        model: &mut Self::State,
        _: Sender<Self::Event>,
    ) -> ComponentOutcome {
        match msg {
            Msg::Set(value) => *model = value,
        };
        ComponentOutcome::Changed
    }
    fn update(&self, model: &Self::State) {
        self.progress.set_value(*model as u32);
        self.spinner.update(*model);
        self.slider.update(*model);
    }
    fn new(prt: &impl CompositeExt, sender: Sender<Self::Event>) -> Self {
        const WIDTH: i32 = 90;
        let group = foxtk::GroupBox::new(prt, "Counter");
        let column = foxtk::VerticalFrame::new(&group);
        let spinner_row = foxtk::HorizontalFrame::new(&column);
        let spinner = foxtk::Spinner::new(&spinner_row)
            .with_range(0, 8)
            .with_callback({
                let sender = sender.clone();
                move |wgt| {
                    if wgt.has_focus() {
                        sender.send(Msg::Set(wgt.value())).unwrap();
                    }
                    false
                }
            });
        let slider_row = foxtk::HorizontalFrame::new(&column);
        let progress = foxtk::ProgressBar::new(&slider_row).with_total(8);
        let slider = foxtk::Slider::new(&slider_row)
            .with_width(WIDTH)
            .with_range(0, 8)
            .with_callback({
                let sender = sender.clone();
                move |wgt| {
                    if wgt.has_focus() {
                        sender.send(Msg::Set(wgt.value())).unwrap();
                    }
                    false
                }
            });
        Self {
            progress,
            slider,
            spinner,
        }
    }
}
