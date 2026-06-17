#![forbid(unsafe_code)]

use foxtk::prelude::*;
use std::sync::mpsc::Sender;

pub enum Msg {
    Draw,
    Clear,
}

#[derive(Default)]
pub struct Scribble {
    canvas: foxtk::Canvas,
    dc: Option<foxtk::Canvas>,
}

impl Component for Scribble {
    type Event = Msg;
    type State = usize;

    fn handle(msg: Self::Event, _model: &mut Self::State, _: Sender<Self::Event>) -> bool {
        match msg {
            Msg::Draw => true,
            Msg::Clear => true,
        }
    }

    fn update(&self, _model: &Self::State) {}

    fn view(&mut self, prt: &impl CompositeExt, sender: Sender<Self::Event>) {
        let frame = foxtk::VerticalFrame::new(prt).with_layout(Layout::Fill);
        frame.inside(|prt| {
            let h = foxtk::HorizontalFrame::new(prt).with_layout(Layout::FillX);
            h.inside(|prt| {
                foxtk::Button::new(prt, "Draw Random").with_callback({
                    let sender = sender.clone();
                    move |_| {
                        sender.send(Msg::Draw).unwrap();
                        false
                    }
                });
                foxtk::Button::new(prt, "Clear").with_callback({
                    let sender = sender.clone();
                    move |_| {
                        sender.send(Msg::Clear).unwrap();
                        false
                    }
                });
            });
            self.canvas = foxtk::Canvas::new(prt).with_layout(Layout::Fill);
            // create a DC for drawing (canvas implements DCWindowExt)
            self.dc = Some(self.canvas.clone());
        });

        // Wire mouse events to draw on the canvas
        let canvas = self.canvas.clone();
        // shared mutable state for last point and whether button is down
        use std::cell::RefCell;
        use std::rc::Rc;
        let state = Rc::new(RefCell::new((0i32, 0i32, false)));
        let s1 = state.clone();
        canvas.set_mouse_callback(move |c, code, x, y| {
            let mut st = s1.borrow_mut();
            match code {
                1 => {
                    // left button press
                    st.0 = x;
                    st.1 = y;
                    st.2 = true;
                }
                2 => {
                    // left button release
                    st.2 = false;
                }
                3 => {
                    // motion
                    if st.2 {
                        // draw line from previous to current
                        c.dc_set_foreground(Color::from_rgb(0, 0, 0));
                        c.dc_set_line_width(2);
                        c.dc_draw_line(st.0, st.1, x, y);
                        st.0 = x;
                        st.1 = y;
                    }
                }
                _ => {}
            }
            false
        });
    }
}

fn main() {
    // small runner to show example
    println!("Run this example via the workspace examples or integrate into your app.");
}
