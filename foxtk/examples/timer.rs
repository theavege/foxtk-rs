#![forbid(unsafe_code)]

use foxtk::prelude::*;
use std::sync::mpsc::Sender;
use std::time::{Duration, Instant};

pub enum Msg {
    Start,
    Stop,
    Reset,
    Tick,
}

#[derive(Default)]
pub struct Timer {
    display: foxtk::Label,
    status: foxtk::Text,
}

#[derive(Default)]
pub struct TimerState {
    running: bool,
    elapsed: Duration,
    started_at: Option<Instant>,
    message: String,
}

impl TimerState {
    fn start(&mut self) {
        if !self.running {
            self.running = true;
            self.started_at = Some(Instant::now());
            self.message = "Running".to_string();
        }
    }

    fn stop(&mut self) {
        if self.running {
            if let Some(started_at) = self.started_at {
                self.elapsed += Instant::now() - started_at;
            }
            self.running = false;
            self.started_at = None;
            self.message = "Stopped".to_string();
        }
    }

    fn reset(&mut self) {
        self.elapsed = Duration::ZERO;
        self.started_at = if self.running { Some(Instant::now()) } else { None };
        self.message = "Reset".to_string();
    }

    fn tick(&mut self) {
        if self.running {
            self.message = "Running".to_string();
        }
    }

    fn display_text(&self) -> String {
        let total = if self.running {
            self.elapsed + self.started_at.map_or(Duration::ZERO, |start| Instant::now() - start)
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

impl Component for Timer {
    type Event = Msg;
    type State = TimerState;

    fn handle(msg: Self::Event, model: &mut Self::State, _: Sender<Self::Event>) -> bool {
        match msg {
            Msg::Start => model.start(),
            Msg::Stop => model.stop(),
            Msg::Reset => model.reset(),
            Msg::Tick => model.tick(),
        }
        true
    }

    fn update(&self, model: &Self::State) {
        self.display.set_text(&model.display_text());
        self.status.set_text(&model.message);
    }

    fn view(&mut self, prt: &impl CompositeExt, sender: Sender<Self::Event>) {
        let root = foxtk::VerticalFrame::new(prt)
            .with_layout(Layout::Fill)
            .with_frame(FrameStyle::Thick);

        root.inside(|prt| {
            self.display = foxtk::Label::new(prt, "00:00.000")
                .with_layout(Layout::FillX)
                .with_height(60);

            let buttons = foxtk::HorizontalFrame::new(prt).with_layout(Layout::FillX);
            buttons.inside(|prt| {
                foxtk::Button::new(prt, "Start").set_callback({
                    let sender = sender.clone();
                    move |_| {
                        sender.send(Msg::Start).unwrap();
                        false
                    }
                });
                foxtk::Button::new(prt, "Stop").set_callback({
                    let sender = sender.clone();
                    move |_| {
                        sender.send(Msg::Stop).unwrap();
                        false
                    }
                });
                foxtk::Button::new(prt, "Reset").set_callback({
                    let sender = sender.clone();
                    move |_| {
                        sender.send(Msg::Reset).unwrap();
                        false
                    }
                });
            });

            self.status = foxtk::Text::new(prt)
                .with_editable(false)
                .with_text("Ready");
        });

        let app = prt.app();
        app.add_timeout(50, move |_| {
            sender.send(Msg::Tick).unwrap();
            true
        });
    }
}

fn main() {
    Timer::run("7GUIs Timer", "FOX", "Timer", 320, 200);
}
