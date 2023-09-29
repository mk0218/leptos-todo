use leptos::*;

#[derive(Clone)]
pub struct TodoItem {
    pub id: usize,
    pub done: RwSignal<bool>,
    pub title: RwSignal<String>,
}

impl TodoItem {
    pub fn new(id: usize, title: &str) -> Self {
        TodoItem {
            id,
            done: create_rw_signal(false),
            title: create_rw_signal(title.to_owned()),
        }
    }
}

#[derive(Clone)]
pub struct TodoList {
    pub items: Vec<TodoItem>,
    next_id: usize,
}

impl TodoList {
    pub fn new() -> Self {
        TodoList {
            items: vec![],
            next_id: 0,
        }
    }

    fn next_id(&mut self) -> usize {
        self.next_id += 1;
        self.next_id - 1
    }

    pub fn add(&mut self, title: &str) {
        let item = TodoItem::new(self.next_id(), title);
        self.items.push(item);
    }
}