#![forbid(unsafe_code)]

use foxtk::prelude::*;
use std::sync::mpsc::Sender;

pub enum Msg {
    FilterChanged(String),
    Create,
    Read,
    Update,
    Delete,
    SelectItem(i32),
}

#[derive(Clone, Debug)]
pub struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    fn display(&self) -> String {
        format!("{}, {}", self.last_name, self.first_name)
    }
}

#[derive(Default)]
pub struct Crud {
    list: foxtk::ListBox,
    filter_field: foxtk::TextField,
    first_name_field: foxtk::TextField,
    last_name_field: foxtk::TextField,
    status: foxtk::Text,
}

pub struct CrudState {
    people: Vec<Person>,
    filter: String,
    selected_index: i32,
    message: String,
}

impl Default for CrudState {
    fn default() -> Self {
        Self {
            people: vec![
                Person {
                    first_name: "John".to_string(),
                    last_name: "Doe".to_string(),
                },
                Person {
                    first_name: "Jane".to_string(),
                    last_name: "Smith".to_string(),
                },
                Person {
                    first_name: "Bob".to_string(),
                    last_name: "Johnson".to_string(),
                },
            ],
            filter: String::new(),
            selected_index: -1,
            message: "Ready".to_string(),
        }
    }
}

impl CrudState {
    fn filtered_people(&self) -> Vec<&Person> {
        self.people
            .iter()
            .filter(|p| p.last_name.to_lowercase().starts_with(&self.filter.to_lowercase()))
            .collect()
    }

    fn update_filter(&mut self, filter: String) {
        self.filter = filter;
        self.selected_index = -1;
        self.message = "Filter updated".to_string();
    }

    fn create(&mut self, first_name: String, last_name: String) {
        if !first_name.is_empty() && !last_name.is_empty() {
            self.people.push(Person { first_name, last_name });
            self.message = "Created".to_string();
        } else {
            self.message = "Invalid input".to_string();
        }
    }

    fn read(&mut self, index: i32) {
        let filtered = self.filtered_people();
        if index >= 0 && (index as usize) < filtered.len() {
            self.selected_index = index;
            self.message = "Selected".to_string();
        } else {
            self.message = "No selection".to_string();
        }
    }

    fn update(&mut self, first_name: String, last_name: String) {
        let filtered = self.filtered_people();
        if self.selected_index >= 0 && (self.selected_index as usize) < filtered.len() {
            if let Some(person) = filtered.get(self.selected_index as usize) {
                let original_idx = self.people.iter().position(|p| {
                    p.first_name == person.first_name && p.last_name == person.last_name
                });
                if let Some(idx) = original_idx {
                    self.people[idx].first_name = first_name;
                    self.people[idx].last_name = last_name;
                    self.message = "Updated".to_string();
                }
            }
        } else {
            self.message = "No selection to update".to_string();
        }
    }

    fn delete(&mut self) {
        let filtered = self.filtered_people();
        if self.selected_index >= 0 && (self.selected_index as usize) < filtered.len() {
            if let Some(person) = filtered.get(self.selected_index as usize) {
                self.people.retain(|p| {
                    !(p.first_name == person.first_name && p.last_name == person.last_name)
                });
                self.selected_index = -1;
                self.message = "Deleted".to_string();
            }
        } else {
            self.message = "No selection to delete".to_string();
        }
    }
}

impl Component for Crud {
    type Event = Msg;
    type State = CrudState;

    fn handle(msg: Self::Event, model: &mut Self::State, _: Sender<Self::Event>) -> bool {
        match msg {
            Msg::FilterChanged(f) => model.update_filter(f),
            Msg::Create => {
                let first = self.first_name_field.text();
                let last = self.last_name_field.text();
                model.create(first, last);
            }
            Msg::Read => model.read(model.selected_index),
            Msg::Update => {
                let first = self.first_name_field.text();
                let last = self.last_name_field.text();
                model.update(first, last);
            }
            Msg::Delete => model.delete(),
            Msg::SelectItem(idx) => model.read(idx),
        }
        true
    }

    fn update(&self, model: &Self::State) {
        self.status.set_text(&model.message);
        self.list.clear_items();

        let filtered = model.filtered_people();
        for person in &filtered {
            self.list.append_item(&person.display());
        }

        if model.selected_index >= 0 && (model.selected_index as usize) < filtered.len() {
            if let Some(person) = filtered.get(model.selected_index as usize) {
                self.first_name_field.set_text(&person.first_name);
                self.last_name_field.set_text(&person.last_name);
            }
        }
    }

    fn view(&mut self, prt: &impl CompositeExt, sender: Sender<Self::Event>) {
        let root = foxtk::VerticalFrame::new(prt)
            .with_layout(Layout::Fill)
            .with_frame(FrameStyle::Thick);

        root.inside(|prt| {
            foxtk::Label::new(prt, "Filter by last name:").with_layout(Layout::FillX);
            self.filter_field = foxtk::TextField::new(prt).with_callback({
                let sender = sender.clone();
                move |wgt| {
                    sender.send(Msg::FilterChanged(wgt.text())).unwrap();
                    false
                }
            });

            foxtk::Label::new(prt, "People:").with_layout(Layout::FillX);
            self.list = foxtk::ListBox::new(prt)
                .with_layout(Layout::Fill)
                .with_height(150)
                .set_num_visible(5);
            self.list.set_callback({
                let sender = sender.clone();
                move |wgt| {
                    let idx = wgt.current_item();
                    sender.send(Msg::SelectItem(idx)).unwrap();
                    false
                }
            });

            foxtk::Label::new(prt, "First name:").with_layout(Layout::FillX);
            self.first_name_field = foxtk::TextField::new(prt).with_layout(Layout::FillX);

            foxtk::Label::new(prt, "Last name:").with_layout(Layout::FillX);
            self.last_name_field = foxtk::TextField::new(prt).with_layout(Layout::FillX);

            let buttons = foxtk::HorizontalFrame::new(prt).with_layout(Layout::FillX);
            buttons.inside(|prt| {
                foxtk::Button::new(prt, "Create").set_callback({
                    let sender = sender.clone();
                    move |_| {
                        sender.send(Msg::Create).unwrap();
                        false
                    }
                });
                foxtk::Button::new(prt, "Update").set_callback({
                    let sender = sender.clone();
                    move |_| {
                        sender.send(Msg::Update).unwrap();
                        false
                    }
                });
                foxtk::Button::new(prt, "Delete").set_callback({
                    let sender = sender.clone();
                    move |_| {
                        sender.send(Msg::Delete).unwrap();
                        false
                    }
                });
            });

            self.status = foxtk::Text::new(prt)
                .with_editable(false)
                .with_height(60)
                .with_text("Ready");
        });
    }
}

fn main() {
    Crud::run("CRUD", "FOX", "CRUD", 400, 500);
}
