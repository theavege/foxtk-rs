//! 7GUIs circle drawer: click to add a circle, right-click to resize it,
//! with one undo step per completed change.

mod models {
    pub const DEFAULT_DIAMETER: i32 = 40;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct Circle {
        pub x: i32,
        pub y: i32,
        pub d: i32,
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    enum Edit {
        Add(Circle),
        Resize { index: usize, from: i32, to: i32 },
    }

    #[derive(Clone, Debug, Default)]
    pub struct Model {
        pub circles: Vec<Circle>,
        undo: Vec<Edit>,
        redo: Vec<Edit>,
    }

    impl Model {
        pub fn add(&mut self, x: i32, y: i32) {
            let circle = Circle {
                x,
                y,
                d: DEFAULT_DIAMETER,
            };
            self.circles.push(circle);
            self.undo.push(Edit::Add(circle));
            self.redo.clear();
        }

        pub fn resize(&mut self, index: usize, from: i32, to: i32) {
            if from == to || self.circles.get(index).is_none() {
                return;
            }
            self.circles[index].d = to;
            self.undo.push(Edit::Resize { index, from, to });
            self.redo.clear();
        }

        pub fn undo(&mut self) {
            let Some(edit) = self.undo.pop() else {
                return;
            };
            match edit {
                Edit::Add(_) => {
                    self.circles.pop();
                }
                Edit::Resize { index, from, .. } => {
                    if let Some(circle) = self.circles.get_mut(index) {
                        circle.d = from;
                    }
                }
            }
            self.redo.push(edit);
        }

        pub fn redo(&mut self) {
            let Some(edit) = self.redo.pop() else {
                return;
            };
            match edit {
                Edit::Add(circle) => self.circles.push(circle),
                Edit::Resize { index, to, .. } => {
                    if let Some(circle) = self.circles.get_mut(index) {
                        circle.d = to;
                    }
                }
            }
            self.undo.push(edit);
        }

        pub fn can_undo(&self) -> bool {
            !self.undo.is_empty()
        }

        pub fn can_redo(&self) -> bool {
            !self.redo.is_empty()
        }
    }

    /// The nearest circle whose center is closer than its radius.
    pub fn hit(circles: &[Circle], x: i32, y: i32) -> Option<usize> {
        let mut best = None;
        let mut best_distance = i64::MAX;
        for (index, circle) in circles.iter().enumerate() {
            let dx = x as i64 - circle.x as i64;
            let dy = y as i64 - circle.y as i64;
            let radius = circle.d as i64 / 2;
            let distance = dx * dx + dy * dy;
            if distance < radius * radius && distance < best_distance {
                best = Some(index);
                best_distance = distance;
            }
        }
        best
    }

    #[cfg(test)]
    mod tests {
        use super::{Circle, DEFAULT_DIAMETER, Model, hit};

        #[test]
        fn undo_and_redo_cover_creation_and_one_diameter_change() {
            let mut model = Model::default();
            model.add(10, 20);
            assert_eq!(model.circles[0].d, DEFAULT_DIAMETER);
            model.resize(0, DEFAULT_DIAMETER, 80);
            assert!(model.can_undo());
            model.undo();
            assert_eq!(model.circles[0].d, DEFAULT_DIAMETER);
            model.undo();
            assert!(model.circles.is_empty());
            assert!(!model.can_undo());
            model.redo();
            model.redo();
            assert_eq!(model.circles[0].d, 80);
            model.add(1, 1);
            assert!(!model.can_redo());
            model.resize(0, 80, 80);
            assert_eq!(model.undo.len(), 3);
        }

        #[test]
        fn the_nearest_circle_under_the_pointer_is_selected() {
            let circles = [Circle { x: 0, y: 0, d: 20 }, Circle { x: 8, y: 0, d: 20 }];
            assert_eq!(hit(&circles, 1, 0), Some(0));
            assert_eq!(hit(&circles, 7, 0), Some(1));
            assert_eq!(hit(&circles, 100, 0), None);
        }
    }
}

use foxtk::prelude::*;
use std::cell::{Cell, RefCell};
use std::rc::Rc;

const FULL_CIRCLE: i32 = 360 * 64;

pub enum Msg {
    Add(i32, i32),
    Resize { index: usize, from: i32, to: i32 },
    Undo,
    Redo,
}

#[derive(Default)]
struct Live {
    circles: Vec<models::Circle>,
    hover: Option<usize>,
}

#[derive(Default)]
pub struct Circles {
    canvas: foxtk::Canvas,
    undo: foxtk::Button,
    redo: foxtk::Button,
    live: Rc<RefCell<Live>>,
}

impl Component for Circles {
    type Event = Msg;
    type State = models::Model;

    fn handle(
        msg: Self::Event,
        model: &mut Self::State,
        _sender: Sender<Self::Event>,
    ) -> ComponentOutcome {
        match msg {
            Msg::Add(x, y) => model.add(x, y),
            Msg::Resize { index, from, to } => model.resize(index, from, to),
            Msg::Undo => model.undo(),
            Msg::Redo => model.redo(),
        }
        ComponentOutcome::Changed
    }

    fn update(&self, model: &Self::State) {
        let mut live = self.live.borrow_mut();
        live.circles = model.circles.clone();
        if live.hover.is_some_and(|index| index >= live.circles.len()) {
            live.hover = None;
        }
        drop(live);
        self.undo.set_enable(model.can_undo());
        self.redo.set_enable(model.can_redo());
        self.canvas.refresh();
    }

    fn view(&mut self, parent: &impl CompositeExt, sender: Sender<Self::Event>) {
        let live = self.live.clone();
        foxtk::VerticalFrame::new(parent)
            .with_layout(Layout::Fill)
            .inside(|parent| {
                foxtk::HorizontalFrame::new(parent).inside(|parent| {
                    self.undo = foxtk::Button::new(parent, "Undo").with_callback({
                        let sender = sender.clone();
                        move |_| {
                            let _ = sender.send(Msg::Undo);
                            false
                        }
                    });
                    self.redo = foxtk::Button::new(parent, "Redo").with_callback({
                        let sender = sender.clone();
                        move |_| {
                            let _ = sender.send(Msg::Redo);
                            false
                        }
                    });
                    self.undo.set_enable(false);
                    self.redo.set_enable(false);
                });
                self.canvas = foxtk::Canvas::new(parent);
                self.canvas.set_layout(Layout::Fill);
                let canvas = self.canvas.alias();
                let menu = foxtk::MenuPane::new(&canvas);
                let chosen = Rc::new(Cell::new(None::<usize>));
                let chosen_for_menu = chosen.clone();
                let live_for_menu = live.clone();
                let canvas_for_menu = canvas.alias();
                foxtk::MenuCommand::new(&menu, "Adjust diameter..").with_callback({
                    let sender = sender.clone();
                    move |_| {
                        if let Some(index) = chosen_for_menu.get() {
                            let from = live_for_menu.borrow().circles.get(index).map(|c| c.d);
                            if let Some(from) = from {
                                let to =
                                    adjust_diameter(&canvas_for_menu, &live_for_menu, index, from);
                                let _ = sender.send(Msg::Resize { index, from, to });
                            }
                        }
                        false
                    }
                });
                let menu = menu.alias();
                let live = live.clone();
                let sender = sender.clone();
                canvas.set_mouse_callback(
                    move |canvas, code, x, y, _, _, width, height| match code {
                        1 => {
                            let miss = {
                                let guard = live.borrow();
                                models::hit(&guard.circles, x, y).is_none()
                            };
                            if miss {
                                let _ = sender.send(Msg::Add(x, y));
                            }
                            true
                        }
                        3 => {
                            let hover = {
                                let guard = live.borrow();
                                models::hit(&guard.circles, x, y)
                            };
                            if live.borrow().hover != hover {
                                live.borrow_mut().hover = hover;
                                canvas.refresh();
                            }
                            true
                        }
                        4 => {
                            let hit = {
                                let guard = live.borrow();
                                models::hit(&guard.circles, x, y)
                            };
                            if let Some(index) = hit {
                                chosen.set(Some(index));
                                live.borrow_mut().hover = Some(index);
                                let (root_x, root_y) = canvas.translate_to_root(x, y);
                                menu.popup(&canvas, root_x, root_y);
                            }
                            true
                        }
                        6 => {
                            let guard = live.borrow();
                            paint(&canvas, &guard, x, y, width, height);
                            true
                        }
                        _ => false,
                    },
                );
            });
    }
}

fn adjust_diameter(
    owner: &foxtk::Canvas,
    live: &Rc<RefCell<Live>>,
    index: usize,
    from: i32,
) -> i32 {
    let dialog = foxtk::DialogBox::new(owner, "Adjust diameter");
    let slider = foxtk::Slider::new(&dialog)
        .with_range(2, 300)
        .with_value(from);
    let live_for_slider = live.clone();
    let canvas = owner.alias();
    slider.with_callback(move |slider| {
        if let Some(circle) = live_for_slider.borrow_mut().circles.get_mut(index) {
            circle.d = slider.value();
        }
        canvas.refresh();
        false
    });
    foxtk::Button::new(&dialog, "Close").with_callback(|button| {
        button.accept_dialog();
        false
    });
    let _ = dialog.execute();
    live.borrow()
        .circles
        .get(index)
        .map(|circle| circle.d)
        .unwrap_or(from)
}

fn paint(canvas: &foxtk::Canvas, live: &Live, x: i32, y: i32, width: i32, height: i32) {
    if width <= 0 || height <= 0 {
        return;
    }
    let dc = foxtk::DCWindow::new(canvas);
    dc.set_foreground(canvas.back_color());
    dc.fill_rectangle(x, y, width, height);
    let gray = Color::from_rgb(160, 160, 160);
    let ink = Color::from_rgb(0, 0, 0);
    for (index, circle) in live.circles.iter().enumerate() {
        let left = circle.x - circle.d / 2;
        let top = circle.y - circle.d / 2;
        if live.hover == Some(index) {
            dc.set_foreground(gray);
            dc.fill_arc(left, top, circle.d, circle.d, 0, FULL_CIRCLE);
        }
        dc.set_foreground(ink);
        dc.draw_arc(left, top, circle.d, circle.d, 0, FULL_CIRCLE);
    }
    dc.finish();
}
