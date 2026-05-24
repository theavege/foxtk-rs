mod models {
    #[derive(Default)]
    pub struct Model {}
}

use foxtk::prelude::*;

pub enum Msg {
    Flight(u32),
}

#[derive(Default)]
pub struct Booker {}

impl Component for Booker {
    type Event = Msg;
    type State = models::Model;
    fn update(&self, _model: &Self::State) {}
    fn handle(msg: Self::Event, _model: &mut Self::State, _: Sender<Self::Event>) -> bool {
        match msg {
            Msg::Flight(_value) => {}
        };
        true
    }
    fn view(&mut self, prt: &impl CompositeExt, sender: Sender<Self::Event>) {
        foxtk::VerticalFrame::new(prt).inside(|prt| {
            foxtk::HorizontalFrame::new(prt).inside(|prt| {
                foxtk::Label::new(prt, "Flight");
                foxtk::ListBox::new(prt)
                    .with_items(&["One-way", "Return"])
                    .with_callback({
                        let sender = sender.clone();
                        move |wgt| {
                            sender.send(Msg::Flight(wgt.current_item() as u32)).unwrap();
                            false
                        }
                    });
            });
            foxtk::HorizontalFrame::new(prt).inside(|prt| {
                foxtk::Label::new(prt, "Departure data");
                foxtk::TextField::new(prt).with_callback({
                    let _sender = sender.clone();
                    move |wgt| {
                        if wgt.has_focus() {
                            let _value = wgt.text().parse::<f64>().unwrap_or_default();
                        }
                        false
                    }
                });
            });
            foxtk::HorizontalFrame::new(prt).inside(|prt| {
                foxtk::Label::new(prt, "Return data");
                foxtk::TextField::new(prt).with_callback({
                    let _sender = sender.clone();
                    move |wgt| {
                        if wgt.has_focus() {
                            let _value = wgt.text().parse::<f64>().unwrap_or_default();
                        }
                        false
                    }
                });
            });
            foxtk::Button::new(prt, "4").disable();
        });
    }
}
