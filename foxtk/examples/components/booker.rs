use foxtk::prelude::*;

pub enum Msg {
    SetName(String),
    SetSurname(String),
    Add,
    Update,
    Delete,
    Select(usize),
}

#[derive(Clone)]
pub struct Person {
    pub name: String,
    pub surname: String,
}

#[derive(Default)]
pub struct BookerModel {
    people: Vec<Person>,
    selected: Option<usize>,
    name: String,
    surname: String,
}

impl BookerModel {
    pub fn people(&self) -> &Vec<Person> {
        &self.people
    }
    pub fn selected(&self) -> Option<usize> {
        self.selected
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn surname(&self) -> &str {
        &self.surname
    }
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
    pub fn set_surname(&mut self, surname: String) {
        self.surname = surname;
    }
    pub fn add_person(&mut self) {
        if !self.name.is_empty() && !self.surname.is_empty() {
            self.people.push(Person {
                name: self.name.clone(),
                surname: self.surname.clone(),
            });
            self.name.clear();
            self.surname.clear();
        }
    }
    pub fn update_person(&mut self) {
        if let Some(idx) = self.selected {
            if idx < self.people.len() {
                self.people[idx].name = self.name.clone();
                self.people[idx].surname = self.surname.clone();
                self.name.clear();
                self.surname.clear();
                self.selected = None;
            }
        }
    }
    pub fn delete_person(&mut self) {
        if let Some(idx) = self.selected {
            if idx < self.people.len() {
                self.people.remove(idx);
                self.selected = None;
            }
        }
    }
    pub fn select_person(&mut self, idx: usize) {
        if idx < self.people.len() {
            self.selected = Some(idx);
            self.name = self.people[idx].name.clone();
            self.surname = self.people[idx].surname.clone();
        }
    }
}

pub type BookerState = BookerModel;

#[derive(Default)]
pub struct BookerExample {
    name_field: foxtk::TextField,
    surname_field: foxtk::TextField,
    list: Option<foxtk::ListBox>,
    add_btn: foxtk::Button,
    update_btn: foxtk::Button,
    delete_btn: foxtk::Button,
}

impl Component for BookerExample {
    type Event = Msg;
    type State = BookerState;
    fn handle(msg: Self::Event, model: &mut Self::State, _: Sender<Self::Event>) -> bool {
        match msg {
            Msg::SetName(name) => model.set_name(name),
            Msg::SetSurname(surname) => model.set_surname(surname),
            Msg::Add => model.add_person(),
            Msg::Update => model.update_person(),
            Msg::Delete => model.delete_person(),
            Msg::Select(idx) => model.select_person(idx),
        };
        true
    }
    fn update(&self, model: &Self::State) {
        self.name_field.set_text(model.name());
        self.surname_field.set_text(model.surname());
        if let Some(ref list) = self.list {
            list.clear_items();
            for person in model.people() {
                list.append_item(&format!("{} {}", person.name, person.surname));
            }
            if let Some(idx) = model.selected() {
                list.set_current_item(idx as i32);
            }
        }
    }
    fn view(&mut self, parent: &impl WindowExt, sender: Sender<Self::Event>) {
        foxtk::VerticalFrame::new(parent).inside(|vbox| {
            foxtk::Button::new(vbox, "Name:");
            self.name_field = foxtk::TextField::new(vbox, 20);
            self.name_field.set_callback({
                let sender = sender.clone();
                move |tf: foxtk::TextField| {
                    sender.send(Msg::SetName(tf.text())).unwrap();
                    false
                }
            });
            foxtk::Button::new(vbox, "Surname:");
            self.surname_field = foxtk::TextField::new(vbox, 20);
            self.surname_field.set_callback({
                let sender = sender.clone();
                move |tf: foxtk::TextField| {
                    sender.send(Msg::SetSurname(tf.text())).unwrap();
                    false
                }
            });
            foxtk::HorizontalFrame::new(vbox).inside(|hbox| {
                self.add_btn = foxtk::Button::new(hbox, "Create").with_callback({
                    let sender = sender.clone();
                    move |_| {
                        sender.send(Msg::Add).unwrap();
                        false
                    }
                });
                self.update_btn = foxtk::Button::new(hbox, "Update").with_callback({
                    let sender = sender.clone();
                    move |_| {
                        sender.send(Msg::Update).unwrap();
                        false
                    }
                });
                self.delete_btn = foxtk::Button::new(hbox, "Delete").with_callback({
                    let sender = sender.clone();
                    move |_| {
                        sender.send(Msg::Delete).unwrap();
                        false
                    }
                });
            });
            self.list = Some(foxtk::ListBox::new(vbox));
            if let Some(ref list) = self.list {
                list.set_callback({
                    let sender = sender.clone();
                    move |lb: foxtk::ListBox| {
                        sender.send(Msg::Select(lb.current_item() as usize)).unwrap();
                        false
                    }
                });
            }
        });
    }
}