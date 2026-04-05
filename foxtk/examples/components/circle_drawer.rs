use foxtk::prelude::*;

pub enum Msg {
    SetDiameter(i32),
    DrawCircle,
}

#[derive(Default)]
pub struct CircleDrawerState {
    diameter: i32,
    circles: Vec<(f32, f32, f32)>, // x, y, radius
}

impl CircleDrawerState {
    pub fn new() -> Self {
        Self {
            diameter: 50,
            circles: Vec::new(),
        }
    }
    pub fn set_diameter(&mut self, diameter: i32) {
        self.diameter = diameter;
    }
    pub fn draw_circle(&mut self) {
        // Draw at center for simplicity
        self.circles.push((200.0, 200.0, self.diameter as f32 / 2.0));
    }
    pub fn circles(&self) -> &Vec<(f32, f32, f32)> {
        &self.circles
    }
    pub fn diameter(&self) -> i32 {
        self.diameter
    }
}

pub type CircleDrawerModel = CircleDrawerState;

#[derive(Default)]
pub struct CircleDrawerExample {
    canvas: Option<foxtk::Canvas>,
    diameter_slider: foxtk::RangeSlider,
    draw_button: foxtk::Button,
}

impl Component for CircleDrawerExample {
    type Event = Msg;
    type State = CircleDrawerModel;
    fn handle(msg: Self::Event, model: &mut Self::State, _: Sender<Self::Event>) -> bool {
        match msg {
            Msg::SetDiameter(d) => model.set_diameter(d),
            Msg::DrawCircle => model.draw_circle(),
        };
        true
    }
    fn update(&self, _model: &Self::State) {
        self.diameter_slider.set_value(_model.diameter());
    }
    fn view(&mut self, parent: &impl WindowExt, sender: Sender<Self::Event>) {
        foxtk::VerticalFrame::new(parent).inside(|vbox| {
            foxtk::HorizontalFrame::new(vbox).inside(|hbox| {
                foxtk::Button::new(hbox, "Diameter:");
                self.diameter_slider = foxtk::RangeSlider::new(hbox);
                self.diameter_slider.set_range(10, 200);
                self.diameter_slider.set_value(50);
                self.diameter_slider.set_callback({
                    let sender = sender.clone();
                    move |slider: foxtk::RangeSlider| {
                        sender.send(Msg::SetDiameter(slider.value())).unwrap();
                        false
                    }
                });
            });
            self.draw_button = foxtk::Button::new(vbox, "Draw Circle").with_callback({
                let sender = sender.clone();
                move |_| {
                    sender.send(Msg::DrawCircle).unwrap();
                    false
                }
            });
            self.canvas = Some(foxtk::Canvas::new(vbox));
        });
    }
}