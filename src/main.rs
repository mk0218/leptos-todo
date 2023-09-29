use leptos::*;

use crate::components::TodoList;

pub mod components;


fn main() { mount_to_body(|| view! { <App/> }) }


#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
        <div class="app">
            <h1 class="title">"TODO"</h1>
            <TodoList/>
        </div>
    }
}