mod data;

use leptos::*;
use data::{TodoItem, TodoList};

#[component]
pub fn TodoItem(
    item: TodoItem,
) -> impl IntoView {
    view! {
        <div class="item row">
            <input
                type="checkbox"
                checked={item.done}
            />
            <p class:item_done=move || item.done.get()>
                { item.title }
            </p>
        </div>
    }
}

#[component]
pub fn TodoList() -> impl IntoView {
    let (todos, set_todos) = create_signal(TodoList::new());

    // A list of filtered todos which is not done yet.
    let todo_list = move || {
        todos.with(|todos| todos.items.iter().filter(|item| {
            !item.done.get()
        }).cloned().collect::<Vec<_>>())
    };

    // A list of filtered todos which is finished.
    let done_list = move || {
        todos.with(|todos| todos.items.iter().filter(|item| {
            item.done.get()
        }).cloned().collect::<Vec<_>>())
    };

    // Event listeners to add todo items when enter key or submit pressed.
    let input_ref = create_node_ref::<html::Input>();
    let add_todo = move || {
        let input = input_ref.get().unwrap();
        let title = input.value();
        let title = title.trim();
        if !title.is_empty() {
            set_todos.update(|list| list.add(&title));
            input.set_value("");
        }
    };
    
    let ev_enter = move |ev: web_sys::KeyboardEvent| {
        ev.stop_propagation();
        if ev.key() == "Enter" {
            add_todo();
        }
    };

    let ev_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        add_todo();
    };

    // Some sample todo items.
    set_todos.update(|list| list.add("세탁물 맡기기"));
    set_todos.update(|list| list.add("화분 물 주기"));
    set_todos.update(|list| list.add("무언가 제목이 긴 할일로 할일의 내용이 박스보다 긴 경우를 테스트하기 위해 추가한 항목"));

    view! {
        // Counters for not done / done todos.
        <div class="row counter">
            <p>
                "Todo: " { move || todo_list().len() } " / "
                "Done: " { move || done_list().len() }
            </p>
        </div>

        // An input field for add new todo items.
        <form on:submit=ev_submit>
            <div class="add row">
                <input
                    type="text"
                    placeholder="Enter to add"
                    autofocus
                    node_ref=input_ref
                    on:keydown=ev_enter
                />
                <input type="submit" value="Add"/>
            </div>
        </form>

        // The Todo list.
        // Todos which are not yet done come first, and then done todos come.
        <ul class="list">
            <For
                each=move || todo_list()
                key=|item| item.id
                view=move |item| {
                    let on_change = move |ev| item.done.set(event_target_checked(&ev));
                    view! {
                        <li>
                            <TodoItem item on:change=on_change/>
                        </li>
                    }
                }
            />

            <For
                each=move || done_list()
                key=|item| item.id
                view=move |item| {
                    let on_change = move |ev| item.done.set(event_target_checked(&ev));
                    view! {
                        <li><TodoItem item on:change=on_change/></li>
                    }
                }
            />
        </ul>
    }
}