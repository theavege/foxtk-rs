mod models {
    #[derive(Default)]
    pub struct Model {
        shotstr: f64,
        targstr: f64,
        targvol: f64,
        aromavol: f64,
    }

    impl Model {
        pub fn shotstr(&self) -> f64 {
            self.shotstr
        }
        pub fn limit(&self) -> f64 {
            self.targvol - self.calculate_nic()
        }
        fn calculate_nic(&self) -> f64 {
            match self.shotstr {
                0.0 => self.shotstr,
                _ => (self.targvol * self.targstr) / self.shotstr,
            }
        }
        pub fn set_shotstr(&mut self, value: f64) {
            self.shotstr = value;
        }
        pub fn set_targstr(&mut self, value: f64) {
            self.targstr = value;
        }
        pub fn set_targvol(&mut self, value: f64) {
            self.targvol = value;
        }
        pub fn set_aromavol(&mut self, value: f64) {
            self.aromavol = value;
        }
        pub fn output(&self) -> [(&str, f64); 4] {
            let shots = self.calculate_nic();
            [
                ("Nicotine Base", shots),
                ("Base", self.targvol - (shots + self.aromavol)),
                ("Flavour", self.aromavol),
                ("Total", self.targvol),
            ]
        }
    }
}

use foxtk::prelude::*;

pub enum Msg {
    Shotstr(f64),
    Targstr(f64),
    Targvol(f64),
    Aromavol(f64),
}

#[derive(Default)]
pub struct NicCalc {
    base: foxtk::ProgressBar,
    nicotine_base: foxtk::ProgressBar,
    flavour: foxtk::ProgressBar,
    total: foxtk::ProgressBar,
    list: foxtk::List,
}

impl Component for NicCalc {
    type Event = Msg;
    type State = models::Model;
    fn update(&self, model: &Self::State) {
        let [nb, b, f, t] = model.output();
        self.base.set_value((t.1 / 100.0 * b.1) as u32);
        self.flavour.set_value((t.1 / 100.0 * f.1) as u32);
        self.nicotine_base.set_value((t.1 / 100.0 * nb.1) as u32);
        self.total.set_value(t.1 as u32);
        self.list.clear_items();
        self.list.append_item("Ingredient: Amount(ml)");
        for (x, y) in model.output() {
            self.list.append_item(&format!("{x}: {y}"));
        }
    }
    fn handle(msg: Self::Event, model: &mut Self::State, _: Sender<Self::Event>) -> bool {
        match msg {
            Msg::Shotstr(value) => {
                if (0f64..1000f64).contains(&value) {
                    model.set_shotstr(value);
                }
            }
            Msg::Targstr(value) => {
                if (0f64..=model.shotstr()).contains(&value) {
                    model.set_targstr(value);
                }
            }
            Msg::Targvol(value) => {
                if (0f64..=100000f64).contains(&value) {
                    model.set_targvol(value);
                }
            }
            Msg::Aromavol(value) => {
                if (0f64..=model.limit()).contains(&value) {
                    model.set_aromavol(value);
                }
            }
        };
        true
    }
    fn view(&mut self, prt: &impl CompositeExt, sender: Sender<Self::Event>) {
        const WIDTH: i32 = 200;
        const PAD: i32 = 10;
        foxtk::VerticalFrame::new(prt)
            .with_pad(0)
            .with_spacing(PAD)
            .inside(|prt| {
                foxtk::HorizontalFrame::new(prt)
                    .with_pad(0)
                    .with_spacing(0)
                    .inside(|prt| {
                        foxtk::Label::new(prt, "Nicotine base strength (mg/ml):").with_width(WIDTH);
                        foxtk::TextField::new(prt).with_callback({
                            let sender = sender.clone();
                            move |wgt| {
                                if wgt.has_focus() {
                                    let value = wgt.text().parse::<f64>().unwrap_or_default();
                                    sender.send(Msg::Shotstr(value)).unwrap();
                                }
                                false
                            }
                        });
                    });
                foxtk::HorizontalFrame::new(prt)
                    .with_pad(0)
                    .with_spacing(0)
                    .inside(|prt| {
                        foxtk::Label::new(prt, "Nicotine strength wanted (mg/ml):")
                            .with_width(WIDTH)
                            .set_justify(Justify::Right);
                        foxtk::TextField::new(prt).with_callback({
                            let sender = sender.clone();
                            move |wgt| {
                                if wgt.has_focus() {
                                    let value = wgt.text().parse::<f64>().unwrap_or_default();
                                    sender.send(Msg::Targstr(value)).unwrap();
                                }
                                false
                            }
                        });
                    });
                foxtk::HorizontalFrame::new(prt)
                    .with_pad(0)
                    .with_spacing(0)
                    .inside(|prt| {
                        foxtk::Label::new(prt, "Amount wanted (ml):")
                            .with_width(WIDTH)
                            .set_justify(Justify::Right);
                        foxtk::TextField::new(prt).with_callback({
                            let sender = sender.clone();
                            move |wgt| {
                                if wgt.has_focus() {
                                    let value = wgt.text().parse::<f64>().unwrap_or_default();
                                    sender.send(Msg::Targvol(value)).unwrap();
                                }
                                false
                            }
                        });
                    });
                foxtk::HorizontalFrame::new(prt)
                    .with_pad(0)
                    .with_spacing(0)
                    .inside(|prt| {
                        foxtk::Label::new(prt, "Flavour amount (ml):")
                            .with_width(WIDTH)
                            .set_justify(Justify::Right);
                        foxtk::TextField::new(prt).with_callback({
                            let sender = sender.clone();
                            move |wgt| {
                                if wgt.has_focus() {
                                    let value = wgt.text().parse::<f64>().unwrap_or_default();
                                    sender.send(Msg::Aromavol(value)).unwrap();
                                }
                                false
                            }
                        });
                    });
                foxtk::HorizontalFrame::new(prt)
                    .with_pad(0)
                    .with_spacing(0)
                    .inside(|prt| {
                        foxtk::Label::new(prt, "Nicotin base")
                            .with_width(WIDTH)
                            .set_justify(Justify::Right);
                        self.nicotine_base = foxtk::ProgressBar::new(prt).with_total(100);
                    });
                foxtk::HorizontalFrame::new(prt)
                    .with_pad(0)
                    .with_spacing(0)
                    .inside(|prt| {
                        foxtk::Label::new(prt, "Base")
                            .with_width(WIDTH)
                            .set_justify(Justify::Right);
                        self.base = foxtk::ProgressBar::new(prt).with_total(100);
                    });
                foxtk::HorizontalFrame::new(prt)
                    .with_pad(0)
                    .with_spacing(0)
                    .inside(|prt| {
                        foxtk::Label::new(prt, "Flavour")
                            .with_width(WIDTH)
                            .set_justify(Justify::Right);
                        self.flavour = foxtk::ProgressBar::new(prt).with_total(100);
                    });
                foxtk::HorizontalFrame::new(prt)
                    .with_pad(0)
                    .with_spacing(0)
                    .inside(|prt| {
                        foxtk::Label::new(prt, "Total")
                            .with_width(WIDTH)
                            .set_justify(Justify::Right);
                        self.total = foxtk::ProgressBar::new(prt).with_total(100);
                    });
                self.list = foxtk::List::new(prt).with_layout(Layout::Fill);
            });
    }
}
