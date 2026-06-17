#![forbid(unsafe_code)]

use foxtk::prelude::*;
use std::{cell::RefCell, fs, rc::Rc, sync::mpsc::Sender};

pub enum Msg {
    Changed,
    SetStatus(String),
}

#[derive(Default)]
pub struct Adie {
    editor: foxtk::Text,
    status: foxtk::Label,
    current_file: Rc<RefCell<String>>,
    dirty: Rc<RefCell<bool>>,
}

pub struct AdieState {
    status: String,
}

impl Default for AdieState {
    fn default() -> Self {
        Self {
            status: "Ready".to_string(),
        }
    }
}

impl AdieState {
    fn set_status(&mut self, message: String) {
        self.status = message;
    }
}

impl Component for Adie {
    type Event = Msg;
    type State = AdieState;

    fn handle(msg: Self::Event, model: &mut Self::State, _: Sender<Self::Event>) -> bool {
        match msg {
            Msg::Changed => model.set_status("Modified".to_string()),
            Msg::SetStatus(message) => model.set_status(message),
        }
        true
    }

    fn update(&self, model: &Self::State) {
        self.status.set_text(&model.status);
    }

    fn view(&mut self, prt: &impl CompositeExt, sender: Sender<Self::Event>) {
        let root_frame = foxtk::VerticalFrame::new(prt)
            .with_layout(Layout::Fill)
            .with_frame(FrameStyle::Thick);

        let window = prt.root();
        let file_path = self.current_file.clone();
        let dirty = self.dirty.clone();
        let editor_handle: Rc<RefCell<Option<foxtk::Text>>> = Rc::new(RefCell::new(None));

        root_frame.inside(|prt| {
            let toolbar = foxtk::HorizontalFrame::new(prt)
                .with_layout(Layout::FillX)
                .with_height(40);

            toolbar.inside(|prt| {
                foxtk::Button::new(prt, "New").set_callback({
                    let editor_handle = editor_handle.clone();
                    let file_path = file_path.clone();
                    let dirty = dirty.clone();
                    let sender = sender.clone();
                    move |_| {
                        if let Some(editor) = editor_handle.borrow().as_ref() {
                            editor.set_text("");
                        }
                        *file_path.borrow_mut() = String::new();
                        *dirty.borrow_mut() = false;
                        sender.send(Msg::SetStatus("New document".to_string())).unwrap();
                        false
                    }
                });

                foxtk::Button::new(prt, "Open").set_callback({
                    let editor_handle = editor_handle.clone();
                    let file_path = file_path.clone();
                    let dirty = dirty.clone();
                    let sender = sender.clone();
                    let window = window.clone();
                    move |_| {
                        let file_name = window.open_file_dialog("Open File", "", "*", 0);
                        if !file_name.is_empty() {
                            match fs::read_to_string(&file_name) {
                                Ok(contents) => {
                                    if let Some(editor) = editor_handle.borrow().as_ref() {
                                        editor.set_text(&contents);
                                    }
                                    *file_path.borrow_mut() = file_name.clone();
                                    *dirty.borrow_mut() = false;
                                    sender.send(Msg::SetStatus(format!("Opened {}", file_name))).unwrap();
                                }
                                Err(err) => {
                                    sender.send(Msg::SetStatus(format!("Open failed: {}", err))).unwrap();
                                }
                            }
                        }
                        false
                    }
                });

                foxtk::Button::new(prt, "Save").set_callback({
                    let editor_handle = editor_handle.clone();
                    let file_path = file_path.clone();
                    let dirty = dirty.clone();
                    let sender = sender.clone();
                    let window = window.clone();
                    move |_| {
                        if let Some(editor) = editor_handle.borrow().as_ref() {
                            let path = file_path.borrow().clone();
                            let target = if path.is_empty() {
                                window.save_file_dialog("Save File", "", "*", 0)
                            } else {
                                path
                            };
                            if !target.is_empty() {
                                match fs::write(&target, editor.text()) {
                                    Ok(_) => {
                                        *file_path.borrow_mut() = target.clone();
                                        *dirty.borrow_mut() = false;
                                        sender.send(Msg::SetStatus(format!("Saved {}", target))).unwrap();
                                    }
                                    Err(err) => {
                                        sender.send(Msg::SetStatus(format!("Save failed: {}", err))).unwrap();
                                    }
                                }
                            }
                        }
                        false
                    }
                });

                foxtk::Button::new(prt, "Save As").set_callback({
                    let editor_handle = editor_handle.clone();
                    let file_path = file_path.clone();
                    let dirty = dirty.clone();
                    let sender = sender.clone();
                    let window = window.clone();
                    move |_| {
                        if let Some(editor) = editor_handle.borrow().as_ref() {
                            let target = window.save_file_dialog("Save File As", "", "*", 0);
                            if !target.is_empty() {
                                match fs::write(&target, editor.text()) {
                                    Ok(_) => {
                                        *file_path.borrow_mut() = target.clone();
                                        *dirty.borrow_mut() = false;
                                        sender.send(Msg::SetStatus(format!("Saved {}", target))).unwrap();
                                    }
                                    Err(err) => {
                                        sender.send(Msg::SetStatus(format!("Save failed: {}", err))).unwrap();
                                    }
                                }
                            }
                        }
                        false
                    }
                });
            });

            let editor = foxtk::Text::new(prt)
                .with_layout(Layout::Fill)
                .with_editable(true)
                .with_text("");

            *editor_handle.borrow_mut() = Some(editor.clone());
            self.editor = editor;

            self.status = foxtk::Label::new(prt, "Ready").with_layout(Layout::FillX);
        });

        let text_sender = sender.clone();
        self.editor.set_callback(move |_| {
            *dirty.borrow_mut() = true;
            text_sender.send(Msg::Changed).unwrap();
            false
        });
    }
}

fn main() {
    Adie::run("Adie", "FOX", "Adie", 800, 600);
}
