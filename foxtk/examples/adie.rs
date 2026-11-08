//! Adie, the FOX text editor, as a [`Component`](foxtk::prelude::Component).
//!
//! One window, one buffer. File, edit, and search commands follow Adie, and
//! the toolbar uses the original GIF icons. Syntax coloring, the file tree,
//! bookmarks, and printing are not in this example. Settings and the window
//! rectangle are stored in the FOX registry.

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use foxtk::prelude::*;

const PATTERNS: &str = "All Files (*)\nC and C++ (*.c,*.cpp,*.h,*.hpp)\nRust (*.rs)\nText (*.txt)";
const SEARCH_BACKWARD: u32 = 1;
const SEARCH_WRAP: u32 = 2;
const CLICKED_QUIT: u32 = 5;
const CLICKED_SAVE: u32 = 6;
const UNDO_LIMIT: usize = 100;

#[derive(Clone)]
enum Event {
    Load { path: String, text: String },
    Stored { path: String, text: String },
    Edited(String),
    Undo,
    Redo,
    Find(String),
    FindNext,
    FindPrev,
    Font(String, i32),
    Wrap(bool),
    Saved(Saved),
}

#[derive(Clone)]
struct Saved {
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    wrap: bool,
    font_family: String,
    font_size: i32,
    directory: String,
}

impl Saved {
    fn read(app: &foxtk::App) -> Self {
        Self {
            x: app.read_int("SETTINGS", "x", 50),
            y: app.read_int("SETTINGS", "y", 50),
            w: app.read_int("SETTINGS", "w", 0),
            h: app.read_int("SETTINGS", "h", 0),
            wrap: app.read_int("SETTINGS", "wordwrap", 0) != 0,
            font_family: app.read_string("SETTINGS", "fontfamily", ""),
            font_size: app.read_int("SETTINGS", "fontsize", 0),
            directory: app.read_string("SETTINGS", "directory", "."),
        }
    }
}

#[derive(Clone)]
struct Live {
    path: String,
    text: String,
    saved: String,
    directory: String,
    search: String,
    wrap: bool,
}

impl Default for Live {
    fn default() -> Self {
        Self {
            path: String::new(),
            text: String::new(),
            saved: String::new(),
            directory: ".".into(),
            search: String::new(),
            wrap: false,
        }
    }
}

struct Model {
    text: String,
    saved: String,
    path: String,
    directory: String,
    undo: Vec<String>,
    redo: Vec<String>,
    revision: u32,
    search: String,
    search_back: bool,
    /// `-1` starts at the cursor, `-2` continues from the last hit.
    search_from: i32,
    search_seq: u32,
    wrap: bool,
    font_family: String,
    font_size: i32,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            text: String::new(),
            saved: String::new(),
            path: String::new(),
            directory: ".".into(),
            undo: Vec::new(),
            redo: Vec::new(),
            revision: 1,
            search: String::new(),
            search_back: false,
            search_from: -1,
            search_seq: 0,
            wrap: false,
            font_family: String::new(),
            font_size: 0,
            x: 50,
            y: 50,
            w: 0,
            h: 0,
        }
    }
}

impl Model {
    fn dirty(&self) -> bool {
        self.text != self.saved
    }

    fn note_edit(&mut self, text: String) {
        if text == self.text {
            return;
        }
        self.undo.push(std::mem::replace(&mut self.text, text));
        if self.undo.len() > UNDO_LIMIT {
            self.undo.remove(0);
        }
        self.redo.clear();
    }

    fn undo(&mut self) -> bool {
        let Some(prev) = self.undo.pop() else {
            return false;
        };
        self.redo.push(std::mem::replace(&mut self.text, prev));
        self.revision += 1;
        true
    }

    fn redo(&mut self) -> bool {
        let Some(next) = self.redo.pop() else {
            return false;
        };
        self.undo.push(std::mem::replace(&mut self.text, next));
        self.revision += 1;
        true
    }

    fn load(&mut self, path: String, text: String) {
        if !path.is_empty() {
            self.directory = parent_dir(&path);
        }
        self.path = path;
        self.saved = text.clone();
        self.text = text;
        self.undo.clear();
        self.redo.clear();
        self.revision += 1;
    }

    fn store(&mut self, path: String, text: String) {
        if !path.is_empty() {
            self.directory = parent_dir(&path);
        }
        self.path = path;
        self.text = text.clone();
        self.saved = text;
    }

    fn restore(&mut self, saved: Saved) {
        self.x = saved.x;
        self.y = saved.y;
        self.w = saved.w;
        self.h = saved.h;
        self.wrap = saved.wrap;
        self.font_family = saved.font_family;
        self.font_size = saved.font_size;
        if !saved.directory.is_empty() {
            self.directory = saved.directory;
        }
    }

    fn find(&mut self, text: String, back: bool, from: i32) {
        self.search = text;
        self.search_back = back;
        self.search_from = from;
        self.search_seq += 1;
    }
}

#[derive(Default)]
struct Adie {
    editor: foxtk::Text,
    status: foxtk::StatusBar,
    live: Rc<RefCell<Live>>,
    applying: Rc<Cell<bool>>,
    updates: Cell<u8>,
    placed: Cell<bool>,
    shown: Cell<u32>,
    seen_search: Cell<u32>,
    search_at: Cell<i32>,
}

impl Component for Adie {
    type Event = Event;
    type State = Model;

    fn handle(
        event: Self::Event,
        model: &mut Self::State,
        _: Sender<Self::Event>,
    ) -> ComponentOutcome {
        match event {
            Event::Load { path, text } => model.load(path, text),
            Event::Stored { path, text } => model.store(path, text),
            Event::Edited(text) => model.note_edit(text),
            Event::Undo => {
                if !model.undo() {
                    return ComponentOutcome::Unchanged;
                }
            }
            Event::Redo => {
                if !model.redo() {
                    return ComponentOutcome::Unchanged;
                }
            }
            Event::Find(text) => model.find(text, false, -1),
            Event::FindNext => model.find(model.search.clone(), false, -2),
            Event::FindPrev => model.find(model.search.clone(), true, -2),
            Event::Font(family, size) => {
                model.font_family = family;
                model.font_size = size;
            }
            Event::Wrap(wrap) => model.wrap = wrap,
            Event::Saved(saved) => model.restore(saved),
        }
        ComponentOutcome::Changed
    }

    fn update(&self, model: &Self::State) {
        if self.shown.get() != model.revision {
            self.shown.set(model.revision);
            self.applying.set(true);
            self.editor.set_text(&model.text);
            self.applying.set(false);
        }
        self.editor.set_word_wrap(model.wrap);
        self.editor.set_tab_columns(8);
        if !model.font_family.is_empty() && model.font_size > 0 {
            self.editor.set_font(&model.font_family, model.font_size);
        }
        let mut missed = false;
        if self.seen_search.get() != model.search_seq && !model.search.is_empty() {
            self.seen_search.set(model.search_seq);
            let start = if model.search_from == -2 {
                self.search_at.get()
            } else {
                self.editor.cursor_pos()
            };
            let flags = SEARCH_WRAP
                | if model.search_back {
                    SEARCH_BACKWARD
                } else {
                    0
                };
            if let Some((beg, end)) = self.editor.find(&model.search, start, flags) {
                self.editor.select(beg, (end - beg).max(0));
                self.search_at
                    .set(if model.search_back { beg } else { end });
            } else {
                missed = true;
            }
        }
        let name = display_name(&model.path);
        let mark = if model.dirty() { "*" } else { "" };
        self.editor.set_shell_title(&format!("{name}{mark} - Adie"));
        let flag = if model.dirty() { "Modified" } else { "" };
        let extra = if missed { "  Not found" } else { "" };
        self.status.set_text(&format!(
            "{name}  {}:{}  {flag}{extra}",
            self.editor.cursor_row() + 1,
            self.editor.cursor_column() + 1
        ));
        *self.live.borrow_mut() = Live {
            path: model.path.clone(),
            text: model.text.clone(),
            saved: model.saved.clone(),
            directory: model.directory.clone(),
            search: model.search.clone(),
            wrap: model.wrap,
        };
        self.place(model);
        self.persist(model);
    }

    fn view(&mut self, parent: &impl CompositeExt, sender: Sender<Self::Event>) {
        let app = parent.app();
        let big = foxtk::Icon::from_gif(&app, include_bytes!("icons/adie/big_gif.gif"));
        let mini = foxtk::Icon::from_gif(&app, include_bytes!("icons/adie/small_gif.gif"));
        let new_icon = foxtk::Icon::from_gif(&app, include_bytes!("icons/adie/new_gif.gif"));
        let open_icon = foxtk::Icon::from_gif(&app, include_bytes!("icons/adie/open_gif.gif"));
        let save_icon = foxtk::Icon::from_gif(&app, include_bytes!("icons/adie/save_gif.gif"));
        let save_as_icon = foxtk::Icon::from_gif(&app, include_bytes!("icons/adie/saveas_gif.gif"));
        let cut_icon = foxtk::Icon::from_gif(&app, include_bytes!("icons/adie/cut_gif.gif"));
        let copy_icon = foxtk::Icon::from_gif(&app, include_bytes!("icons/adie/copy_gif.gif"));
        let paste_icon = foxtk::Icon::from_gif(&app, include_bytes!("icons/adie/paste_gif.gif"));
        let delete_icon = foxtk::Icon::from_gif(&app, include_bytes!("icons/adie/delete_gif.gif"));
        let undo_icon = foxtk::Icon::from_gif(&app, include_bytes!("icons/adie/undo_gif.gif"));
        let redo_icon = foxtk::Icon::from_gif(&app, include_bytes!("icons/adie/redo_gif.gif"));
        let search_icon = foxtk::Icon::from_gif(&app, include_bytes!("icons/adie/search_gif.gif"));
        let next_icon =
            foxtk::Icon::from_gif(&app, include_bytes!("icons/adie/searchnext_gif.gif"));
        let prev_icon =
            foxtk::Icon::from_gif(&app, include_bytes!("icons/adie/searchprev_gif.gif"));
        let font_icon = foxtk::Icon::from_gif(&app, include_bytes!("icons/adie/fonts_gif.gif"));
        let help_icon = foxtk::Icon::from_gif(&app, include_bytes!("icons/adie/help_gif.gif"));
        let quit_icon = foxtk::Icon::from_gif(&app, include_bytes!("icons/adie/quit_gif.gif"));
        parent.set_window_icon(&big);
        parent.set_window_mini_icon(&mini);

        let live = self.live.clone();
        let applying = self.applying.clone();
        foxtk::VerticalFrame::new(parent)
            .with_layout(Layout::Fill)
            .inside(|parent| {
                let menu = foxtk::MenuBar::new(parent);
                let file_pane = foxtk::MenuPane::new(parent);
                let edit_pane = foxtk::MenuPane::new(parent);
                let search_pane = foxtk::MenuPane::new(parent);
                let options_pane = foxtk::MenuPane::new(parent);
                let help_pane = foxtk::MenuPane::new(parent);
                foxtk::MenuTitle::new(&menu, "&File", &file_pane);
                foxtk::MenuTitle::new(&menu, "&Edit", &edit_pane);
                foxtk::MenuTitle::new(&menu, "&Search", &search_pane);
                foxtk::MenuTitle::new(&menu, "&Options", &options_pane);
                foxtk::MenuTitle::new(&menu, "&Help", &help_pane);
                let bar = foxtk::ToolBar::new(parent);

                let editor = foxtk::Text::new(parent);
                self.editor = share_text(&editor);
                editor.set_tab_columns(8);
                let editor_for_edits = share_text(&self.editor);
                let applying_for_edits = applying.clone();
                editor.with_callback({
                    let sender = sender.clone();
                    move |_| {
                        if !applying_for_edits.get() {
                            let _ = sender.send(Event::Edited(editor_for_edits.text()));
                        }
                        false
                    }
                });

                file_command(&file_pane, "&New", Some(&new_icon), {
                    let sender = sender.clone();
                    let live = live.clone();
                    let editor = share_text(&self.editor);
                    move |item| {
                        if keep_or_save(&item, &editor, &live, Some(&sender)) {
                            let _ = sender.send(Event::Load {
                                path: String::new(),
                                text: String::new(),
                            });
                        }
                        false
                    }
                });
                file_command(&file_pane, "&Open...", Some(&open_icon), {
                    let sender = sender.clone();
                    let live = live.clone();
                    let editor = share_text(&self.editor);
                    move |item| {
                        if !keep_or_save(&item, &editor, &live, Some(&sender)) {
                            return false;
                        }
                        let directory = live.borrow().directory.clone();
                        let path = item.open_file_dialog("Open File", &directory, PATTERNS, 0);
                        if path.is_empty() {
                            return false;
                        }
                        match std::fs::read(&path) {
                            Ok(bytes) => {
                                let _ = sender.send(Event::Load {
                                    path: path.clone(),
                                    text: String::from_utf8_lossy(&bytes).into_owned(),
                                });
                            }
                            Err(err) => {
                                item.message(
                                    MessageBox::Ok,
                                    &format!("Could not open {path}: {err}"),
                                    Message::Error,
                                );
                            }
                        }
                        false
                    }
                });
                file_command(&file_pane, "&Save", Some(&save_icon), {
                    let sender = sender.clone();
                    let live = live.clone();
                    let editor = share_text(&self.editor);
                    move |item| {
                        save_buffer(&item, &editor, &live, &sender);
                        false
                    }
                });
                file_command(&file_pane, "Save &As...", Some(&save_as_icon), {
                    let sender = sender.clone();
                    let live = live.clone();
                    let editor = share_text(&self.editor);
                    move |item| {
                        save_buffer_as(&item, &editor, &live, &sender);
                        false
                    }
                });
                foxtk::MenuSeparator::new(&file_pane);
                file_command(&file_pane, "&Quit", Some(&quit_icon), {
                    let live = live.clone();
                    let editor = share_text(&self.editor);
                    move |item| {
                        if keep_or_save(&item, &editor, &live, None) {
                            write_settings(&item);
                            item.close_shell();
                        }
                        false
                    }
                });

                edit_command(&edit_pane, "&Undo", &undo_icon, &sender, Event::Undo);
                edit_command(&edit_pane, "&Redo", &redo_icon, &sender, Event::Redo);
                foxtk::MenuSeparator::new(&edit_pane);
                widget_command(
                    &edit_pane,
                    "Cu&t",
                    &cut_icon,
                    &self.editor,
                    foxtk::Text::cut,
                );
                widget_command(
                    &edit_pane,
                    "&Copy",
                    &copy_icon,
                    &self.editor,
                    foxtk::Text::copy,
                );
                widget_command(
                    &edit_pane,
                    "&Paste",
                    &paste_icon,
                    &self.editor,
                    foxtk::Text::paste,
                );
                widget_command(
                    &edit_pane,
                    "&Delete",
                    &delete_icon,
                    &self.editor,
                    foxtk::Text::delete_selection,
                );

                file_command(&search_pane, "&Find...", Some(&search_icon), {
                    let sender = sender.clone();
                    let live = live.clone();
                    move |item| {
                        let dialog = foxtk::SearchDialog::new(&item, "Find");
                        dialog.set_search_text(&live.borrow().search);
                        if dialog.execute() != 0 {
                            let _ = sender.send(Event::Find(dialog.search_text()));
                        }
                        false
                    }
                });
                edit_command(
                    &search_pane,
                    "Find &Next",
                    &next_icon,
                    &sender,
                    Event::FindNext,
                );
                edit_command(
                    &search_pane,
                    "Find &Previous",
                    &prev_icon,
                    &sender,
                    Event::FindPrev,
                );

                file_command(&options_pane, "&Word wrap", None, {
                    let sender = sender.clone();
                    let live = live.clone();
                    move |_| {
                        let _ = sender.send(Event::Wrap(!live.borrow().wrap));
                        false
                    }
                });
                file_command(&options_pane, "&Font...", Some(&font_icon), {
                    let sender = sender.clone();
                    move |item| {
                        let dialog = foxtk::FontDialog::new(&item, "Editor Font");
                        if dialog.execute() != 0 {
                            let (family, size) = dialog.font_selection();
                            let _ = sender.send(Event::Font(family, size));
                        }
                        false
                    }
                });
                file_command(&help_pane, "&Help", Some(&help_icon), |item| {
                    show_help(&item);
                    false
                });

                tool(&bar, &new_icon, "New", {
                    let sender = sender.clone();
                    let live = live.clone();
                    let editor = share_text(&self.editor);
                    move |button| {
                        if keep_or_save(&button, &editor, &live, Some(&sender)) {
                            let _ = sender.send(Event::Load {
                                path: String::new(),
                                text: String::new(),
                            });
                        }
                        false
                    }
                });
                tool(&bar, &open_icon, "Open", {
                    let sender = sender.clone();
                    let live = live.clone();
                    let editor = share_text(&self.editor);
                    move |button| {
                        if !keep_or_save(&button, &editor, &live, Some(&sender)) {
                            return false;
                        }
                        let directory = live.borrow().directory.clone();
                        let path = button.open_file_dialog("Open File", &directory, PATTERNS, 0);
                        if path.is_empty() {
                            return false;
                        }
                        match std::fs::read(&path) {
                            Ok(bytes) => {
                                let _ = sender.send(Event::Load {
                                    path,
                                    text: String::from_utf8_lossy(&bytes).into_owned(),
                                });
                            }
                            Err(err) => {
                                button.message(
                                    MessageBox::Ok,
                                    &format!("Could not open the file: {err}"),
                                    Message::Error,
                                );
                            }
                        }
                        false
                    }
                });
                tool(&bar, &save_icon, "Save", {
                    let sender = sender.clone();
                    let live = live.clone();
                    let editor = share_text(&self.editor);
                    move |button| {
                        save_buffer(&button, &editor, &live, &sender);
                        false
                    }
                });
                tool(&bar, &cut_icon, "Cut", {
                    let editor = share_text(&self.editor);
                    move |_| {
                        editor.cut();
                        false
                    }
                });
                tool(&bar, &copy_icon, "Copy", {
                    let editor = share_text(&self.editor);
                    move |_| {
                        editor.copy();
                        false
                    }
                });
                tool(&bar, &paste_icon, "Paste", {
                    let editor = share_text(&self.editor);
                    move |_| {
                        editor.paste();
                        false
                    }
                });
                tool(&bar, &undo_icon, "Undo", {
                    let sender = sender.clone();
                    move |_| {
                        let _ = sender.send(Event::Undo);
                        false
                    }
                });
                tool(&bar, &redo_icon, "Redo", {
                    let sender = sender.clone();
                    move |_| {
                        let _ = sender.send(Event::Redo);
                        false
                    }
                });
                tool(&bar, &search_icon, "Find", {
                    let sender = sender.clone();
                    let live = live.clone();
                    move |button| {
                        let dialog = foxtk::SearchDialog::new(&button, "Find");
                        dialog.set_search_text(&live.borrow().search);
                        if dialog.execute() != 0 {
                            let _ = sender.send(Event::Find(dialog.search_text()));
                        }
                        false
                    }
                });
                tool(&bar, &next_icon, "Find next", {
                    let sender = sender.clone();
                    move |_| {
                        let _ = sender.send(Event::FindNext);
                        false
                    }
                });
                tool(&bar, &prev_icon, "Find previous", {
                    let sender = sender.clone();
                    move |_| {
                        let _ = sender.send(Event::FindPrev);
                        false
                    }
                });

                self.status = foxtk::StatusBar::new(parent);
            });

        let editor = share_text(&self.editor);
        let live = self.live.clone();
        parent.on_close_if(move |window| {
            if keep_or_save(&window, &editor, &live, None) {
                write_settings(&window);
                false
            } else {
                true
            }
        });
        let _ = sender.send(Event::Saved(Saved::read(&parent.app())));
    }
}

impl Adie {
    fn place(&self, model: &Model) {
        if self.placed.get() || self.updates.get() == 0 {
            return;
        }
        self.placed.set(true);
        self.editor.place_shell(model.x, model.y, model.w, model.h);
    }

    fn persist(&self, model: &Model) {
        if self.updates.get() == 0 {
            self.updates.set(1);
            return;
        }
        let app = self.editor.app();
        let (x, y, w, h) = self.editor.shell_geometry();
        if w > 0 && h > 0 {
            app.write_int("SETTINGS", "x", x);
            app.write_int("SETTINGS", "y", y);
            app.write_int("SETTINGS", "w", w);
            app.write_int("SETTINGS", "h", h);
        }
        app.write_int("SETTINGS", "wordwrap", i32::from(model.wrap));
        app.write_string("SETTINGS", "fontfamily", &model.font_family);
        app.write_int("SETTINGS", "fontsize", model.font_size);
        app.write_string("SETTINGS", "directory", &model.directory);
    }
}

fn share_text(text: &foxtk::Text) -> foxtk::Text {
    unsafe { foxtk::Text::from_raw(text.as_raw()) }
}

fn file_command(
    pane: &impl CompositeExt,
    title: &str,
    icon: Option<&foxtk::Icon>,
    callback: impl FnMut(foxtk::MenuCommand) -> bool + 'static,
) {
    let command = foxtk::MenuCommand::new(pane, title);
    if let Some(icon) = icon {
        command.set_icon(icon);
    }
    command.with_callback(callback);
}

fn edit_command(
    pane: &impl CompositeExt,
    title: &str,
    icon: &foxtk::Icon,
    sender: &Sender<Event>,
    event: Event,
) {
    let sender = sender.clone();
    file_command(pane, title, Some(icon), move |_| {
        let _ = sender.send(event.clone());
        false
    });
}

fn widget_command(
    pane: &impl CompositeExt,
    title: &str,
    icon: &foxtk::Icon,
    editor: &foxtk::Text,
    action: fn(&foxtk::Text),
) {
    let editor = share_text(editor);
    file_command(pane, title, Some(icon), move |_| {
        action(&editor);
        false
    });
}

fn tool(
    parent: &impl CompositeExt,
    icon: &foxtk::Icon,
    tip: &str,
    callback: impl FnMut(foxtk::Button) -> bool + 'static,
) {
    let button = foxtk::Button::new(parent, "").with_icon(icon);
    button.set_text("");
    button.set_layout(Layout::Normal);
    button.set_tip_text(tip);
    button.with_callback(callback);
}

fn parent_dir(path: &str) -> String {
    std::path::Path::new(path)
        .parent()
        .filter(|dir| !dir.as_os_str().is_empty())
        .map(|dir| dir.display().to_string())
        .unwrap_or_else(|| ".".into())
}

fn display_name(path: &str) -> String {
    if path.is_empty() {
        "untitled".into()
    } else {
        std::path::Path::new(path)
            .file_name()
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_else(|| path.to_string())
    }
}

fn keep_or_save(
    window: &impl WindowExt,
    editor: &foxtk::Text,
    live: &Rc<RefCell<Live>>,
    sender: Option<&Sender<Event>>,
) -> bool {
    let current = editor.text();
    if current == live.borrow().saved {
        return true;
    }
    match window.message(
        MessageBox::QuitSaveCancel,
        "Save changes before continuing?",
        Message::Question,
    ) {
        CLICKED_QUIT => true,
        CLICKED_SAVE => {
            if !save_text(window, live, &current, false) {
                return false;
            }
            if let Some(sender) = sender {
                let path = live.borrow().path.clone();
                let _ = sender.send(Event::Stored {
                    path,
                    text: current,
                });
            }
            true
        }
        _ => false,
    }
}

fn save_buffer(
    window: &impl WindowExt,
    editor: &foxtk::Text,
    live: &Rc<RefCell<Live>>,
    sender: &Sender<Event>,
) {
    let current = editor.text();
    let path = live.borrow().path.clone();
    if path.is_empty() {
        save_buffer_as(window, editor, live, sender);
        return;
    }
    if save_text(window, live, &current, false) {
        let _ = sender.send(Event::Stored {
            path,
            text: current,
        });
    }
}

fn save_buffer_as(
    window: &impl WindowExt,
    editor: &foxtk::Text,
    live: &Rc<RefCell<Live>>,
    sender: &Sender<Event>,
) {
    let current = editor.text();
    if save_text(window, live, &current, true) {
        let path = live.borrow().path.clone();
        let _ = sender.send(Event::Stored {
            path,
            text: current,
        });
    }
}

fn save_text(
    window: &impl WindowExt,
    live: &Rc<RefCell<Live>>,
    text: &str,
    ask_path: bool,
) -> bool {
    let path = if ask_path || live.borrow().path.is_empty() {
        let directory = live.borrow().directory.clone();
        let chosen = window.save_file_dialog("Save File", &directory, PATTERNS, 0);
        if chosen.is_empty() {
            return false;
        }
        chosen
    } else {
        live.borrow().path.clone()
    };
    if let Err(err) = std::fs::write(&path, text) {
        window.message(
            MessageBox::Ok,
            &format!("Could not save {path}: {err}"),
            Message::Error,
        );
        return false;
    }
    let mut snapshot = live.borrow().clone();
    snapshot.directory = parent_dir(&path);
    snapshot.path = path;
    snapshot.text = text.to_string();
    snapshot.saved = text.to_string();
    *live.borrow_mut() = snapshot;
    true
}

fn write_settings(window: &impl WindowExt) {
    let app = window.app();
    let (x, y, width, height) = window.shell_geometry();
    if width > 0 && height > 0 {
        app.write_int("SETTINGS", "x", x);
        app.write_int("SETTINGS", "y", y);
        app.write_int("SETTINGS", "w", width);
        app.write_int("SETTINGS", "h", height);
    }
}

fn show_help(owner: &impl WindowExt) {
    let dialog = foxtk::DialogBox::new(owner, "Adie Help");
    let body = foxtk::VerticalFrame::new(&dialog).with_layout(Layout::Fill);
    let text = foxtk::Text::new(&body).with_editable(false);
    text.set_text(HELP);
    let close = foxtk::HorizontalFrame::new(&body);
    foxtk::Button::new(&close, "Close").with_callback(|button| {
        button.accept_dialog();
        false
    });
    let (x, y, _, _) = owner.shell_geometry();
    dialog.position(x + 40, y + 40, 640, 420);
    dialog.execute_placed();
}

const HELP: &str = "\
Adie

New, Open, Save, and Save As edit one buffer. Closing the window, or \
Quit, asks before discarding changes.

Undo and Redo remember the last edits. Cut, Copy, Paste, and Delete use \
the selection. Find searches forward or backward and wraps around.

Word wrap and the editor font are kept in the FOX registry, with the \
window position and the last directory.
";

fn main() {
    Adie::run("Adie", "", "Adie", 760, 520);
}

#[cfg(test)]
mod tests {
    use super::Model;

    #[test]
    fn undo_and_redo_walk_the_edits() {
        let mut model = Model::default();
        model.note_edit("a".into());
        model.note_edit("ab".into());
        assert!(model.undo());
        assert_eq!(model.text, "a");
        assert!(model.redo());
        assert_eq!(model.text, "ab");
        assert!(model.undo());
        assert_eq!(model.text, "a");
    }

    #[test]
    fn saving_clears_the_modified_flag() {
        let mut model = Model::default();
        model.note_edit("hello".into());
        assert!(model.dirty());
        model.store(model.path.clone(), model.text.clone());
        assert!(!model.dirty());
        assert!(model.undo());
        assert!(model.dirty());
    }

    #[test]
    fn loading_drops_the_undo_stack() {
        let mut model = Model::default();
        model.note_edit("hello".into());
        model.load("notes.txt".into(), "fresh".into());
        assert_eq!(model.text, "fresh");
        assert!(!model.dirty());
        assert!(!model.undo());
    }
}
