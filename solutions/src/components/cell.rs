use crate::model::board::Case;
use leptos::prelude::*;

#[component]
pub fn Cell(case: Case) -> impl IntoView {
    if case.is_revealed() && !case.is_mine() {
        view! {
            <div
                class="cell square revealed"
            >
            {case.get_mines_around()}
            </div>
        }.into_any()
    } else {
        view! {
            <button
                class="cell square"
                class:revealed=move || case.is_revealed()
                class:mine=move || case.is_revealed() && case.is_mine()
                class:flag=move || case.is_flagged()
            />
        }.into_any()
    }
}