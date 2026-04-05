use foxtk::prelude::*;

pub enum Msg {
    Start,
    Stop,
    Reset,
    Tick,
}

#[derive(Default)]
pub struct TimerExample {
    progress: foxtk::ProgressBar,
    text: foxtk::TextField,
    start_btn: foxtk::Button,
    stop_btn: foxtk::Button,
    reset_btn: foxtk::Button,
    tick_btn: foxtk::Button,
}

impl Component for TimerExample {
    type Event = Msg;
    type State = TimerState;
    fn handle(msg: Self::Event, model: &mut Self::State, _: Sender<Self::Event>) -> bool {
        match msg {
            Msg::Start => model.start(),
            Msg::Stop => model.stop(),
            Msg::Reset => model.reset(),
            Msg::Tick => model.tick(),
        };
        true
    }
    fn update(&self, model: &Self::State) {
        self.progress.set_progress(model.progress());
        self.text.set_text(&format!("{:.1}s / {:.1}s", model.elapsed, model.duration));
    }
    fn view(&mut self, parent: &impl WindowExt, sender: Sender<Self::Event>) {
        foxtk::VerticalFrame::new(parent).inside(|vbox| {
            self.text = foxtk::TextField::new(vbox, 20);
            self.progress = foxtk::ProgressBar::new(vbox);
            foxtk::HorizontalFrame::new(vbox).inside(|hbox| {
                self.start_btn = foxtk::Button::new(hbox, "Start").with_callback({
                    let sender = sender.clone();
                    move |_| {
                        sender.send(Msg::Start).unwrap();
                        false
                    }
                });
                self.stop_btn = foxtk::Button::new(hbox, "Stop").with_callback({
                    let sender = sender.clone();
                    move |_| {
                        sender.send(Msg::Stop).unwrap();
                        false
                    }
                });
                self.reset_btn = foxtk::Button::new(hbox, "Reset").with_callback({
                    let sender = sender.clone();
                    move |_| {
                        sender.send(Msg::Reset).unwrap();
                        false
                    }
                });
                self.tick_btn = foxtk::Button::new(hbox, "Tick").with_callback({
                    let sender = sender.clone();
                    move |_| {
                        sender.send(Msg::Tick).unwrap();
                        false
                    }
                });
            });
        });
    }
}

#[derive(Default)]
pub struct TimerState {
    elapsed: f32,
    duration: f32,
    running: bool,
}

impl TimerState {
    pub fn new() -> Self {
        Self {
            elapsed: 0.0,
            duration: 10.0,
            running: false,
        }
    }
    pub fn progress(&self) -> u32 {
        (self.elapsed / self.duration * 100.0) as u32
    }
    pub fn start(&mut self) {
        self.running = true;
    }
    pub fn stop(&mut self) {
        self.running = false;
    }
    pub fn reset(&mut self) {
        self.elapsed = 0.0;
        self.running = false;
    }
    pub fn tick(&mut self) {
        if self.running && self.elapsed < self.duration {
            self.elapsed += 1.0;
        }
    }
}