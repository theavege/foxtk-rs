use foxtk::prelude::*;

pub enum Msg {
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
            Msg::Add(value) => *model += value,
        };
        true
    }
    fn update(&self, model: &Self::State) {
        self.spinner.set_value(*model);
        self.label.set_text(&model.to_string());
    }
    fn view(&mut self, parent: &impl CompositeExt, sender: Sender<Self::Event>) {
        foxtk::VerticalFrame::new(parent).inside(|vbox| {
            foxtk::HorizontalFrame::new(vbox).inside(|hbox| {
                foxtk::Button::new(hbox, "Prev").set_callback({
                    let sender = sender.clone();
                    move |_| {
                        sender.send(Msg::Add(-1)).unwrap();
                        false
                    }
                });
                self.label = foxtk::Label::new(hbox, "");
                foxtk::Button::new(hbox, "Next").set_callback({
                    let sender = sender.clone();
                    move |_| {
                        sender.send(Msg::Add(1)).unwrap();
                        false
                    }
                });
            });
            self.spinner = foxtk::Spinner::new(vbox, 6)
                .with_range(0, 8)
                .with_increment(1);
            self.progress = foxtk::ProgressBar::new(vbox)
                .with_total(8);
            self.slider = foxtk::Slider::new(vbox)
                .with_range(0, 8)
                .with_increment(1);
        });
    }
}
