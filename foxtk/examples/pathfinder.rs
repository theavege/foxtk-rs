#![forbid(unsafe_code)]

use foxtk::prelude::*;
use std::sync::mpsc::Sender;

pub enum Msg {
    AddPoint(i32, i32),
    Clear,
    ToggleGrid,
}

#[derive(Default)]
pub struct Pathfinder {
    canvas: foxtk::Canvas,
    status: foxtk::Text,
    grid: bool,
}

#[derive(Default)]
pub struct PathfinderState {
    points: Vec<(i32, i32)>,
    message: String,
    grid: bool,
}

impl PathfinderState {
    fn add_point(&mut self, x: i32, y: i32) {
        self.points.push((x, y));
        self.message = format!("Points: {}", self.points.len());
    }

    fn clear(&mut self) {
        self.points.clear();
        self.message = "Cleared".to_string();
    }

    fn toggle_grid(&mut self) {
        self.grid = !self.grid;
        self.message = if self.grid { "Grid: On" } else { "Grid: Off" }.to_string();
    }
}

impl Component for Pathfinder {
    type Event = Msg;
    type State = PathfinderState;

    fn handle(msg: Self::Event, model: &mut Self::State, _: Sender<Self::Event>) -> bool {
        match msg {
            Msg::AddPoint(x, y) => model.add_point(x, y),
            Msg::Clear => model.clear(),
            Msg::ToggleGrid => model.toggle_grid(),
        }
        true
    }

    fn update(&self, model: &Self::State) {
        self.status.set_text(&model.message);
        let dc = self.canvas.new_dc();
        dc.dc_set_foreground(Color::from_rgb(255, 255, 255));
        dc.dc_fill_rect(0, 0, 400, 360);

        if model.grid {
            dc.dc_set_foreground(Color::from_rgb(220, 220, 220));
            for i in (0..400).step_by(20) {
                dc.dc_draw_line(i, 0, i, 360);
            }
            for i in (0..360).step_by(20) {
                dc.dc_draw_line(0, i, 400, i);
            }
        }

        if !model.points.is_empty() {
            dc.dc_set_foreground(Color::from_rgb(0, 0, 0));
            let mut iter = model.points.iter();
            if let Some(&(x0, y0)) = iter.next() {
                let mut prev = (x0, y0);
                dc.dc_draw_point(x0, y0);
                for &(x, y) in iter {
                    dc.dc_draw_line(prev.0, prev.1, x, y);
                    dc.dc_draw_point(x, y);
                    prev = (x, y);
                }
            }
        }
    }

    fn view(&mut self, prt: &impl CompositeExt, sender: Sender<Self::Event>) {
        let root = foxtk::VerticalFrame::new(prt).with_layout(Layout::Fill);

        root.inside(|prt| {
            self.status = foxtk::Text::new(prt)
                .with_editable(false)
                .with_text("Click the canvas to add path points.");

            let button_bar = foxtk::HorizontalFrame::new(prt).with_layout(Layout::FillX);
            button_bar.inside(|prt| {
                foxtk::Button::new(prt, "Clear").set_callback({
                    let sender = sender.clone();
                    move |_| {
                        sender.send(Msg::Clear).unwrap();
                        false
                    }
                });
                foxtk::Button::new(prt, "Toggle Grid").set_callback({
                    let sender = sender.clone();
                    move |_| {
                        sender.send(Msg::ToggleGrid).unwrap();
                        false
                    }
                });
            });

            self.canvas = foxtk::Canvas::new(prt).with_layout(Layout::Fill).with_width(400).with_height(360);
        });

        self.canvas.set_mouse_callback(move |_canvas, code, x, y| {
            if code == 1 {
                sender.send(Msg::AddPoint(x, y)).unwrap();
            }
            false
        });
    }
}

fn main() {
    Pathfinder::run("FOX Pathfinder", "FOX", "Pathfinder", 420, 460);
}
