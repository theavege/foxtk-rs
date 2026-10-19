use std::path::PathBuf;

use foxtk::prelude::*;

#[derive(Default)]
pub struct Model {
    content: String,
    saved_content: String,
    path: Option<PathBuf>,
    dirty: bool,
    status: String,
}

impl Model {
    fn edit(&mut self, content: String) {
        if self.content != content {
            self.content = content;
            self.dirty = self.content != self.saved_content;
            self.status = if self.dirty {
                "Modified".into()
            } else {
                "Ready".into()
            };
        }
    }

    fn open(&mut self, path: PathBuf) {
        if self.dirty {
            self.status = "Save changes before opening another file".into();
            return;
        }
        match std::fs::read_to_string(&path) {
            Ok(content) => {
                self.content = content.clone();
                self.saved_content = content;
                self.path = Some(path);
                self.dirty = false;
                self.status = "Opened".into();
            }
            Err(error) => self.status = format!("Open failed: {error}"),
        }
    }

    fn save(&mut self) {
        let Some(path) = self.path.clone() else {
            self.status = "Use Save As to choose a file".into();
            return;
        };
        self.save_to(path);
    }

    fn save_to(&mut self, path: PathBuf) {
        match std::fs::write(&path, self.content.as_bytes()) {
            Ok(()) => {
                self.saved_content.clone_from(&self.content);
                self.path = Some(path);
                self.dirty = false;
                self.status = "Saved".into();
            }
            Err(error) => self.status = format!("Save failed: {error}"),
        }
    }

    fn title(&self) -> String {
        match &self.path {
            Some(path) => path
                .file_name()
                .map(|name| name.to_string_lossy().into_owned())
                .unwrap_or_else(|| path.display().to_string()),
            None => "Untitled".into(),
        }
    }

    fn status(&self) -> &str {
        if self.status.is_empty() {
            "Ready"
        } else {
            &self.status
        }
    }
}

pub enum Event {
    Edited(String),
    Open(PathBuf),
    Save,
    SaveAs(PathBuf),
}

impl Model {
    fn handle(&mut self, event: Event) {
        match event {
            Event::Edited(content) => self.edit(content),
            Event::Open(path) => self.open(path),
            Event::Save => self.save(),
            Event::SaveAs(path) => self.save_to(path),
        }
    }
}

#[derive(Default)]
pub struct Adie {
    title: foxtk::Label,
    editor: foxtk::Text,
    status: foxtk::Label,
    save: foxtk::Button,
}

impl Component for Adie {
    type Event = Event;
    type State = Model;

    fn handle(event: Self::Event, model: &mut Self::State, _: Sender<Self::Event>) -> bool {
        model.handle(event);
        true
    }

    fn update(&self, model: &Self::State) {
        self.title.set_text(&model.title());
        self.status.set_text(model.status());
        self.save.set_enable(model.dirty && model.path.is_some());
        if !self.editor.has_focus() && self.editor.text() != model.content {
            self.editor.set_text(&model.content);
        }
    }

    fn view(&mut self, parent: &impl CompositeExt, sender: Sender<Self::Event>) {
        foxtk::VerticalFrame::new(parent)
            .with_layout(Layout::Fill)
            .with_spacing(4)
            .inside(|parent| {
                foxtk::HorizontalFrame::new(parent)
                    .with_layout(Layout::FillX)
                    .with_spacing(4)
                    .inside(|parent| {
                        foxtk::Button::new(parent, "Open").with_callback({
                            let sender = sender.clone();
                            move |button| {
                                let path =
                                    button.open_file_dialog("Open", ".", "*.txt,*.rs,*.md,*", 0);
                                if !path.is_empty()
                                    && let Err(error) =
                                        sender.send(Event::Open(PathBuf::from(path)))
                                {
                                    eprintln!("Editor event was dropped: {error}");
                                }
                                false
                            }
                        });
                        self.save = foxtk::Button::new(parent, "Save").with_callback({
                            let sender = sender.clone();
                            move |_| {
                                if let Err(error) = sender.send(Event::Save) {
                                    eprintln!("Editor event was dropped: {error}");
                                }
                                false
                            }
                        });
                        foxtk::Button::new(parent, "Save As").with_callback({
                            let sender = sender.clone();
                            move |button| {
                                let path =
                                    button.save_file_dialog("Save As", ".", "*.txt,*.rs,*.md,*", 0);
                                if !path.is_empty()
                                    && let Err(error) =
                                        sender.send(Event::SaveAs(PathBuf::from(path)))
                                {
                                    eprintln!("Editor event was dropped: {error}");
                                }
                                false
                            }
                        });
                        self.title =
                            foxtk::Label::new(parent, "Untitled").with_layout(Layout::FillX);
                    });
                self.editor = foxtk::Text::new(parent)
                    .with_layout(Layout::Fill)
                    .with_editable(true)
                    .with_font("monospace", 12)
                    .with_callback({
                        let sender = sender.clone();
                        move |editor| {
                            if editor.has_focus()
                                && let Err(error) = sender.send(Event::Edited(editor.text()))
                            {
                                eprintln!("Editor event was dropped: {error}");
                            }
                            false
                        }
                    });
                self.status = foxtk::Label::new(parent, "Ready").with_layout(Layout::FillX);
            });
    }
}

#[cfg(test)]
mod tests {
    use super::{Event, Model};
    use std::{
        fs,
        path::PathBuf,
        time::{SystemTime, UNIX_EPOCH},
    };

    fn temp_file() -> PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!("foxtk-adie-test-{nonce}.txt"))
    }

    #[test]
    fn save_and_reopen_round_trip_document_contents() {
        let path = temp_file();
        let mut model = Model::default();
        model.handle(Event::Edited("hello\nFOX".into()));
        model.handle(Event::SaveAs(path.clone()));

        assert!(!model.dirty);
        assert_eq!(fs::read_to_string(&path).unwrap(), "hello\nFOX");

        let mut reopened = Model::default();
        reopened.handle(Event::Open(path.clone()));
        assert_eq!(reopened.content, "hello\nFOX");
        assert_eq!(reopened.path.as_ref(), Some(&path));

        fs::remove_file(path).unwrap();
    }

    #[test]
    fn failed_open_preserves_the_current_document_and_reports_the_error() {
        let path = temp_file();
        let mut model = Model::default();
        model.handle(Event::Edited("saved".into()));
        model.handle(Event::SaveAs(path.clone()));
        fs::remove_file(path).unwrap();
        model.handle(Event::Open(temp_file()));

        assert_eq!(model.content, "saved");
        assert!(!model.dirty);
        assert!(model.status.starts_with("Open failed:"));
    }

    #[test]
    fn opening_another_file_does_not_discard_unsaved_changes() {
        let mut model = Model::default();
        model.handle(Event::Edited("unsaved".into()));
        model.handle(Event::Open(temp_file()));

        assert_eq!(model.content, "unsaved");
        assert!(model.dirty);
        assert_eq!(model.status(), "Save changes before opening another file");
    }

    #[test]
    fn reverting_to_saved_contents_clears_the_dirty_state() {
        let path = temp_file();
        let mut model = Model::default();
        model.handle(Event::Edited("saved".into()));
        model.handle(Event::SaveAs(path.clone()));
        model.handle(Event::Edited("changed".into()));
        model.handle(Event::Edited("saved".into()));

        assert!(!model.dirty);
        fs::remove_file(path).unwrap();
    }
}
