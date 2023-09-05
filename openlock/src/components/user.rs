use leptos::*;

#[derive(Debug, Clone)]
pub struct UserItem {
    pub name: String,
}

#[component]
pub fn UserCard(cx: Scope) -> impl IntoView {
    return view! {cx,
        <div
            hx-get="/user"
            hx-trigger="load delay:1s">

            <p>Connect User</p>
        </div>
    };
}

#[component]
pub fn UserComponent(cx: Scope, user: UserItem) -> impl IntoView {
    return view! {cx,
        <div>
            <p>Name: {user.name}</p>
        </div>
    };
}
