use leptos::*;

#[component]
pub fn Nav(cx: Scope) -> impl IntoView {
    view! {cx,
        <header class="header">
            <nav>
                <a href="/">Home</a>
            </nav>
        </header>
    }
}
