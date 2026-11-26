//! 7GUIs timer: a gauge, an elapsed-time label, a duration slider, and reset.
//!
//! The gauge runs until the elapsed time reaches the duration. Lengthening
//! the duration starts it again. Reset brings the elapsed time back to zero.

mod models {
    #[derive(Clone, Debug)]
    pub struct Model {
        pub elapsed_ms: u64,
        pub duration_ms: u64,
        pub armed: bool,
    }

    impl Default for Model {
        fn default() -> Self {
            Self {
                elapsed_ms: 0,
                duration_ms: 15_000,
                armed: false,
            }
        }
    }

    impl Model {
        pub fn running(&self) -> bool {
            self.elapsed_ms < self.duration_ms
        }

        pub fn advance(&mut self, step_ms: u64) {
            if self.running() {
                self.elapsed_ms = (self.elapsed_ms + step_ms).min(self.duration_ms);
            }
        }

        pub fn set_duration_secs(&mut self, seconds: i32) {
            self.duration_ms = (seconds.max(0) as u64).saturating_mul(1000);
        }

        pub fn duration_secs(&self) -> i32 {
            (self.duration_ms / 1000) as i32
        }

        pub fn reset(&mut self) {
            self.elapsed_ms = 0;
        }

        pub fn elapsed_text(&self) -> String {
            format!("{:.1} s", self.elapsed_ms as f64 / 1000.0)
        }

        /// Progress and total for the gauge. A zero duration is already full.
        pub fn gauge(&self) -> (u32, u32) {
            if self.duration_ms == 0 {
                (1, 1)
            } else {
                (
                    self.elapsed_ms.min(self.duration_ms) as u32,
                    self.duration_ms as u32,
                )
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::Model;

        #[test]
        fn it_stops_at_the_duration_and_restarts_when_that_grows() {
            let mut model = Model::default();
            assert!(model.running());
            model.advance(14_000);
            assert!(model.running());
            model.advance(2_000);
            assert_eq!(model.elapsed_ms, 15_000);
            assert!(!model.running());
            assert_eq!(model.gauge(), (15_000, 15_000));
            model.set_duration_secs(20);
            assert!(model.running());
            model.reset();
            assert_eq!(model.elapsed_ms, 0);
            assert_eq!(model.elapsed_text(), "0.0 s");
            model.set_duration_secs(0);
            assert!(!model.running());
            assert_eq!(model.gauge(), (1, 1));
        }
    }
}

use foxtk::prelude::*;
use std::thread;
use std::time::Duration;

const TICK_MS: u64 = 50;

pub enum Msg {
    Start,
    Duration(i32),
    Reset,
    Tick,
}

#[derive(Default)]
pub struct Timer {
    gauge: foxtk::ProgressBar,
    elapsed: foxtk::Label,
    slider: foxtk::Slider,
    duration: foxtk::Label,
}

impl Component for Timer {
    type Event = Msg;
    type State = models::Model;

    fn handle(
        msg: Self::Event,
        model: &mut Self::State,
        sender: Sender<Self::Event>,
    ) -> ComponentOutcome {
        match msg {
            Msg::Start => {}
            Msg::Duration(seconds) => model.set_duration_secs(seconds),
            Msg::Reset => model.reset(),
            Msg::Tick => {
                model.armed = false;
                model.advance(TICK_MS);
            }
        }
        if model.running() && !model.armed {
            model.armed = true;
            thread::spawn(move || {
                thread::sleep(Duration::from_millis(TICK_MS));
                if let Err(error) = sender.send(Msg::Tick) {
                    eprintln!("Timer event was dropped: {error}");
                }
            });
        }
        ComponentOutcome::Changed
    }

    fn update(&self, model: &Self::State) {
        let (progress, total) = model.gauge();
        self.gauge.set_total(total);
        self.gauge.set_progress(progress);
        self.elapsed.set_text(&model.elapsed_text());
        self.slider.update(model.duration_secs());
        self.duration
            .set_text(&format!("{} s", model.duration_secs()));
    }

    fn view(&mut self, parent: &impl CompositeExt, sender: Sender<Self::Event>) {
        foxtk::VerticalFrame::new(parent)
            .with_layout(Layout::Fill)
            .inside(|parent| {
                self.gauge = foxtk::ProgressBar::new(parent).with_total(15_000);
                self.gauge.set_progress(0);
                let elapsed = foxtk::Label::new(parent, "Elapsed");
                elapsed.set_text("0.0 s");
                self.elapsed = elapsed;
                foxtk::HorizontalFrame::new(parent).inside(|parent| {
                    let caption = foxtk::Label::new(parent, "Duration");
                    caption.set_text("Duration");
                    self.slider = foxtk::Slider::new(parent)
                        .with_range(0, 100)
                        .with_value(15)
                        .with_callback({
                            let sender = sender.clone();
                            move |slider| {
                                let _ = sender.send(Msg::Duration(slider.value()));
                                false
                            }
                        });
                    let duration = foxtk::Label::new(parent, "15 s");
                    duration.set_text("15 s");
                    self.duration = duration;
                });
                foxtk::Button::new(parent, "Reset").with_callback({
                    let sender = sender.clone();
                    move |_| {
                        let _ = sender.send(Msg::Reset);
                        false
                    }
                });
            });
        let _ = sender.send(Msg::Start);
    }
}
