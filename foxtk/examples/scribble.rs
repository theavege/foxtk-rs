//! FOX Scribble, as a [`Component`](foxtk::prelude::Component).
//!
//! Drag on the canvas to draw a red line. Paint fills the exposed rectangle
//! with the canvas color, so a covered stroke is gone when that area is
//! shown again. Clear does that for the whole canvas and is enabled only
//! after something has been drawn. Exit closes the window.

use std::cell::Cell;
use std::rc::Rc;

use foxtk::prelude::*;

const LEFT_PRESS: i32 = 1;
const LEFT_RELEASE: i32 = 2;
const MOTION: i32 = 3;
const PAINT: i32 = 6;
const SEPARATOR_GROOVE: u32 = 0x0000_8000;
const SEPARATOR_RIDGE: u32 = 0x0001_0000;

#[derive(Clone, Copy)]
enum Event {
    Down,
    Stroke,
    Up,
    Clear,
}

#[derive(Default)]
struct Model {
    drawing: bool,
    dirty: bool,
}

impl Model {
    fn down(&mut self) {
        self.drawing = true;
    }

    fn stroke(&mut self) {
        if self.drawing {
            self.dirty = true;
        }
    }

    fn up(&mut self) {
        if self.drawing {
            self.dirty = true;
            self.drawing = false;
        }
    }

    fn clear(&mut self) {
        self.drawing = false;
        self.dirty = false;
    }
}

#[derive(Default)]
struct Scribble {
    canvas: foxtk::Canvas,
    clear: foxtk::Button,
    drawing: Rc<Cell<bool>>,
}

impl Component for Scribble {
    type Event = Event;
    type State = Model;

    fn handle(
        event: Self::Event,
        model: &mut Self::State,
        _sender: Sender<Self::Event>,
    ) -> ComponentOutcome {
        match event {
            Event::Down => model.down(),
            Event::Stroke => model.stroke(),
            Event::Up => model.up(),
            Event::Clear => model.clear(),
        }
        ComponentOutcome::Changed
    }

    fn update(&self, model: &Self::State) {
        self.clear.set_enable(model.dirty);
    }

    fn view(&mut self, parent: &impl CompositeExt, sender: Sender<Self::Event>) {
        let contents = foxtk::HorizontalFrame::new(parent);
        contents.set_height(0);

        let canvas_frame = foxtk::VerticalFrame::new(&contents)
            .with_frame(FrameStyle::Sunken)
            .with_pad(10);
        canvas_frame.set_layout(Layout::Fill);
        let canvas_label = foxtk::Label::new(&canvas_frame, "Canvas Frame");
        canvas_label.set_text("Canvas Frame");
        canvas_label.set_layout(Layout::FillX);
        let groove = foxtk::Separator::new(&canvas_frame);
        groove.set_style(SEPARATOR_GROOVE);
        groove.set_layout(Layout::FillX);

        let canvas = foxtk::Canvas::new(&canvas_frame);
        canvas.set_layout(Layout::Fill);
        let drawing = self.drawing.clone();
        let canvas_for_mouse = share_canvas(&canvas);
        let mouse_sender = sender.clone();
        canvas.set_mouse_callback(
            move |_, code, x, y, last_x, last_y, width, height| match code {
                LEFT_PRESS => {
                    canvas_for_mouse.grab();
                    drawing.set(true);
                    let _ = mouse_sender.send(Event::Down);
                    true
                }
                MOTION if drawing.get() => {
                    stroke(&canvas_for_mouse, last_x, last_y, x, y);
                    let _ = mouse_sender.send(Event::Stroke);
                    true
                }
                LEFT_RELEASE => {
                    canvas_for_mouse.ungrab();
                    if drawing.get() {
                        stroke(&canvas_for_mouse, last_x, last_y, x, y);
                        drawing.set(false);
                        let _ = mouse_sender.send(Event::Up);
                    }
                    true
                }
                PAINT => {
                    wipe(&canvas_for_mouse, x, y, width, height);
                    true
                }
                _ => false,
            },
        );
        self.canvas = canvas;

        let button_frame = foxtk::VerticalFrame::new(&contents)
            .with_frame(FrameStyle::Sunken)
            .with_pad(10);
        button_frame.set_layout(Layout::FillY);
        let button_label = foxtk::Label::new(&button_frame, "Button Frame");
        button_label.set_text("Button Frame");
        button_label.set_layout(Layout::FillX);
        let ridge = foxtk::Separator::new(&button_frame);
        ridge.set_style(SEPARATOR_RIDGE);
        ridge.set_layout(Layout::FillX);

        let canvas_for_clear = share_canvas(&self.canvas);
        let drawing = self.drawing.clone();
        let clear_sender = sender.clone();
        self.clear = foxtk::Button::new(&button_frame, "Clear")
            .with_pad(10)
            .with_callback(move |_| {
                wipe(
                    &canvas_for_clear,
                    0,
                    0,
                    canvas_for_clear.width(),
                    canvas_for_clear.height(),
                );
                drawing.set(false);
                let _ = clear_sender.send(Event::Clear);
                false
            });
        foxtk::Button::new(&button_frame, "Exit")
            .with_pad(10)
            .with_callback(|button| {
                button.close_shell();
                false
            });
    }
}

fn stroke(canvas: &foxtk::Canvas, x1: i32, y1: i32, x2: i32, y2: i32) {
    let dc = foxtk::DCWindow::new(canvas);
    dc.set_foreground(Color::from_rgb(255, 0, 0));
    dc.draw_line(x1, y1, x2, y2);
    dc.finish();
}

fn wipe(canvas: &foxtk::Canvas, x: i32, y: i32, width: i32, height: i32) {
    if width <= 0 || height <= 0 {
        return;
    }
    let dc = foxtk::DCWindow::new(canvas);
    dc.set_foreground(canvas.back_color());
    dc.fill_rectangle(x, y, width, height);
    dc.finish();
}

fn share_canvas(canvas: &foxtk::Canvas) -> foxtk::Canvas {
    unsafe { foxtk::Canvas::from_raw(canvas.as_raw()) }
}

fn main() {
    Scribble::run("Scribble", "FoxTest", "Scribble Application", 800, 600);
}

#[cfg(test)]
mod tests {
    use super::Model;

    #[test]
    fn a_stroke_dirties_the_canvas_until_clear() {
        let mut model = Model::default();
        model.down();
        assert!(!model.dirty);
        model.stroke();
        assert!(model.dirty);
        assert!(model.drawing);
        model.up();
        assert!(model.dirty);
        assert!(!model.drawing);
        model.clear();
        assert!(!model.dirty);
        assert!(!model.drawing);
    }

    #[test]
    fn motion_before_the_button_does_not_dirty() {
        let mut model = Model::default();
        model.stroke();
        model.up();
        assert!(!model.dirty);
        assert!(!model.drawing);
    }
}
