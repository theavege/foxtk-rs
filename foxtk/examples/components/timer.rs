mod models {
    use std::time::{Duration, Instant};

    #[derive(Default)]
    pub struct Model {
        pub running: bool,
        elapsed: Duration,
        started_at: Option<Instant>,
        pub message: String,
    }

    impl Model {
        pub fn start(&mut self) {
            if !self.running {
                self.running = true;
                self.started_at = Some(Instant::now());
                self.message = "Running".to_string();
            }
        }

        pub fn stop(&mut self) {
            if self.running {
                if let Some(started_at) = self.started_at {
                    self.elapsed += Instant::now() - started_at;
                }
                self.running = false;
                self.started_at = None;
                self.message = "Stopped".to_string();
            }
        }

        pub fn reset(&mut self) {
            self.elapsed = Duration::ZERO;
            self.started_at = if self.running {
                Some(Instant::now())
            } else {
                None
            };
            self.message = "Reset".to_string();
        }

        pub fn tick(&mut self) {
            if self.running {
                self.message = "Running".to_string();
            }
        }

        pub fn display_text(&self) -> String {
            let total = if self.running {
                self.elapsed
                    + self
                        .started_at
                        .map_or(Duration::ZERO, |start| Instant::now() - start)
            } else {
                self.elapsed
            };
            let seconds = total.as_secs();
            let minutes = seconds / 60;
            let seconds = seconds % 60;
            let millis = total.subsec_millis();
            format!("{:02}:{:02}.{:03}", minutes, seconds, millis)
        }
    }
}
use foxtk::prelude::*;

pub enum Msg {
    Start,
    Stop,
    Reset,
    Tick,
}

#[derive(Default)]
pub struct Timer {
    display: foxtk::TextField,
    status: foxtk::Text,
}

impl Component for Timer {
    type Event = Msg;
    type State = models::Model;

    fn handle(msg: Self::Event, model: &mut Self::State, sender: Sender<Self::Event>) -> bool {
        match msg {
            Msg::Start => model.start(),
            Msg::Stop => model.stop(),
            Msg::Reset => model.reset(),
            Msg::Tick => model.tick(),
        }
        if model.running {
            sender.send(Msg::Tick).unwrap();
        }
        true
    }

    fn update(&self, model: &Self::State) {
        self.display.set_text(&model.display_text());
        self.status.set_text(&model.message);
    }

    fn view(&mut self, prt: &impl CompositeExt, sender: Sender<Self::Event>) {
        foxtk::VerticalFrame::new(prt).inside(|prt| {
            self.display = foxtk::TextField::new(prt)
                .with_height(60)
                .with_editable(false);

            foxtk::HorizontalFrame::new(prt)
                .with_layout(Layout::FillX)
                .inside(|prt| {
                    foxtk::Button::new(prt, "Start").with_callback({
                        let sender = sender.clone();
                        move |_| {
                            sender.send(Msg::Start).unwrap();
                            false
                        }
                    });
                    foxtk::Button::new(prt, "Stop").with_callback({
                        let sender = sender.clone();
                        move |_| {
                            sender.send(Msg::Stop).unwrap();
                            false
                        }
                    });
                    foxtk::Button::new(prt, "Reset").with_callback({
                        let sender = sender.clone();
                        move |_| {
                            sender.send(Msg::Reset).unwrap();
                            false
                        }
                    });
                });
            self.status = foxtk::Text::new(prt).with_editable(false);
        });
    }
}
