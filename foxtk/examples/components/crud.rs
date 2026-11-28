//! 7GUIs CRUD: a prefix filter, a name list, and create, update, and delete.

mod models {
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct Person {
        pub name: String,
        pub surname: String,
    }

    impl Person {
        pub fn label(&self) -> String {
            format!("{}, {}", self.surname, self.name)
        }
    }

    #[derive(Clone, Debug, Default)]
    pub struct Model {
        pub people: Vec<Person>,
        pub prefix: String,
        pub selected: Option<usize>,
    }

    impl Model {
        pub fn visible(&self) -> Vec<usize> {
            self.people
                .iter()
                .enumerate()
                .filter(|(_, person)| person.surname.starts_with(&self.prefix))
                .map(|(index, _)| index)
                .collect()
        }

        pub fn labels(&self) -> Vec<String> {
            self.visible()
                .into_iter()
                .map(|index| self.people[index].label())
                .collect()
        }

        /// Selection index in the filtered list, or -1 when nothing is selected.
        pub fn list_index(&self) -> i32 {
            let Some(selected) = self.selected else {
                return -1;
            };
            self.visible()
                .iter()
                .position(|index| *index == selected)
                .map(|index| index as i32)
                .unwrap_or(-1)
        }

        #[cfg(test)]
        pub fn select_visible(&mut self, index: i32) {
            self.selected = self.visible().get(index as usize).copied();
        }

        pub fn set_prefix(&mut self, prefix: String) {
            self.prefix = prefix;
            if self.list_index() < 0 {
                self.selected = None;
            }
        }

        pub fn create(&mut self, name: String, surname: String) {
            self.people.push(Person { name, surname });
        }

        pub fn update_selected(&mut self, name: String, surname: String) -> bool {
            let Some(index) = self.selected else {
                return false;
            };
            if self.list_index() < 0 {
                return false;
            }
            self.people[index] = Person { name, surname };
            if self.list_index() < 0 {
                self.selected = None;
            }
            true
        }

        pub fn delete_selected(&mut self) -> bool {
            let Some(index) = self.selected else {
                return false;
            };
            if self.list_index() < 0 {
                return false;
            }
            self.people.remove(index);
            self.selected = None;
            true
        }

        pub fn can_modify(&self) -> bool {
            self.list_index() >= 0
        }
    }

    #[cfg(test)]
    mod tests {
        use super::Model;

        #[test]
        fn filter_create_update_and_delete() {
            let mut model = Model::default();
            model.create("Hans".into(), "Emil".into());
            model.create("John".into(), "Smith".into());
            model.create("Max".into(), "Mustermann".into());
            model.set_prefix("M".into());
            assert_eq!(model.labels(), vec!["Mustermann, Max".to_string()]);
            assert!(!model.can_modify());
            model.select_visible(0);
            assert!(model.can_modify());
            assert!(model.update_selected("Max".into(), "Muster".into()));
            assert_eq!(model.people[2].label(), "Muster, Max");
            model.set_prefix(String::new());
            model.select_visible(0);
            assert!(model.delete_selected());
            assert_eq!(model.people.len(), 2);
            assert!(!model.can_modify());
            assert!(!model.delete_selected());
        }
    }
}

use foxtk::prelude::*;
use std::cell::Cell;
use std::rc::Rc;

pub enum Msg {
    Prefix(String),
    Select(i32),
    Create(String, String),
    Update(String, String),
    Delete,
}

pub struct Crud {
    list: foxtk::List,
    update_button: foxtk::Button,
    delete_button: foxtk::Button,
    applying: Rc<Cell<bool>>,
}

impl Component for Crud {
    type Event = Msg;
    type State = models::Model;

    fn handle(
        msg: Self::Event,
        model: &mut Self::State,
        _sender: Sender<Self::Event>,
    ) -> ComponentOutcome {
        match msg {
            Msg::Prefix(prefix) => model.set_prefix(prefix),
            Msg::Select(index) => {
                let next = model.visible().get(index as usize).copied();
                if model.selected == next {
                    return ComponentOutcome::Unchanged;
                }
                model.selected = next;
            }
            Msg::Create(name, surname) => model.create(name, surname),
            Msg::Update(name, surname) => {
                if !model.update_selected(name, surname) {
                    return ComponentOutcome::Unchanged;
                }
            }
            Msg::Delete => {
                if !model.delete_selected() {
                    return ComponentOutcome::Unchanged;
                }
            }
        }
        ComponentOutcome::Changed
    }

    fn update(&self, model: &Self::State) {
        self.applying.set(true);
        self.list.clear_items();
        for label in model.labels() {
            self.list.append_item(&label);
        }
        let current = model.list_index();
        if current >= 0 {
            self.list.set_current_item(current);
        }
        self.applying.set(false);
        let enabled = model.can_modify();
        self.update_button.set_enable(enabled);
        self.delete_button.set_enable(enabled);
    }

    fn new(parent: &impl CompositeExt, sender: Sender<Self::Event>) -> Self {
        let applying = Rc::new(Cell::new(false));
        let root = foxtk::VerticalFrame::new(parent).with_layout(Layout::Fill);
        let prefix_row = foxtk::HorizontalFrame::new(&root);
        let caption = foxtk::Label::new(&prefix_row, "Prefix");
        caption.set_text("Prefix");
        foxtk::TextField::new(&prefix_row).with_callback({
            let sender = sender.clone();
            move |field| {
                if field.has_focus() {
                    let _ = sender.send(Msg::Prefix(field.text()));
                }
                false
            }
        });
        let body = foxtk::HorizontalFrame::new(&root).with_layout(Layout::Fill);
        let list = foxtk::List::new(&body).with_callback({
            let sender = sender.clone();
            let applying = applying.clone();
            move |list| {
                if !applying.get() {
                    let _ = sender.send(Msg::Select(list.current_item()));
                }
                false
            }
        });
        let side = foxtk::VerticalFrame::new(&body).with_layout(Layout::FillY);
        let name_caption = foxtk::Label::new(&side, "Name");
        name_caption.set_text("Name");
        let name = foxtk::TextField::new(&side);
        let surname_caption = foxtk::Label::new(&side, "Surname");
        surname_caption.set_text("Surname");
        let surname = foxtk::TextField::new(&side);
        let name_for_create = name.alias();
        let surname_for_create = surname.alias();
        foxtk::Button::new(&side, "Create").with_callback({
            let sender = sender.clone();
            move |_| {
                let _ = sender.send(Msg::Create(
                    name_for_create.text(),
                    surname_for_create.text(),
                ));
                false
            }
        });
        let name_for_update = name.alias();
        let surname_for_update = surname.alias();
        let update_button = foxtk::Button::new(&side, "Update").with_callback({
            let sender = sender.clone();
            move |_| {
                let _ = sender.send(Msg::Update(
                    name_for_update.text(),
                    surname_for_update.text(),
                ));
                false
            }
        });
        let delete_button = foxtk::Button::new(&side, "Delete").with_callback({
            let sender = sender.clone();
            move |_| {
                let _ = sender.send(Msg::Delete);
                false
            }
        });
        update_button.set_enable(false);
        delete_button.set_enable(false);
        Self {
            list,
            update_button,
            delete_button,
            applying,
        }
    }
}
