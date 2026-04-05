mod models {
    #[derive(Debug, Default)]
    pub struct Model {
        pub cel: Option<f64>,
        pub far: Option<f64>,
    }

    impl Model {
        pub fn set_cel(&mut self, value: f64) {
            self.far = Some((value * 9.0 / 5.0) + 32.0);
            self.cel = None;
        }

        pub fn set_far(&mut self, value: f64) {
            self.cel = Some((value - 32.0) * 5.0 / 9.0);
            self.far = None;
        }
    }
}

use foxtk::prelude::*;

pub enum Msg {
    Cel(f64),
    Far(f64),
}

#[derive(Clone, Default)]
pub struct Converter {
    cel: foxtk::TextField,
    far: foxtk::TextField,
}

impl Component for Converter {
    type Event = Msg;
    type State = models::Model;
    fn handle(msg: Self::Event, model: &mut Self::State, _: Sender<Self::Event>) -> bool {
        match msg {
            Msg::Cel(value) => model.set_cel(value),
            Msg::Far(value) => model.set_far(value),
        };
        true
    }
    fn update(&self, model: &Self::State) {
        if let Some(value) = model.cel {
            self.cel.set_text(&value.to_string());
        }
        if let Some(value) = model.far {
            self.far.set_text(&value.to_string());
        }
    }
    fn view(&mut self, parent: &impl WindowExt, sender: Sender<Self::Event>) {
        let hbox = foxtk::HorizontalFrame::new(parent);
        self.cel = foxtk::TextField::new(&hbox, 16);
        self.cel.set_callback({
            let sender = sender.clone();
            move |wgt| {
                let value = wgt.text().parse::<f64>().unwrap_or_default();
                sender.send(Msg::Cel(value)).unwrap();
                false
            }
        });
        self.far = foxtk::TextField::new(&hbox, 16);
        self.far.set_callback({
            let sender = sender.clone();
            move |wgt| {
                let value = wgt.text().parse::<f64>().unwrap_or_default();
                sender.send(Msg::Far(value)).unwrap();
                false
            }
        });
    }
}
