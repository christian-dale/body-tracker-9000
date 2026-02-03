use dioxus::prelude::*;
use crate::components::partials::wrapper::Wrapper;
use crate::components::views::home::Home;
use crate::components::views::add_entry::AddEntry;

#[derive(Routable, Clone)]
pub enum Route {
    #[layout(Wrapper)]
        #[route("/")]
        Home {},
        #[route("/add-entry")]
        AddEntry {}
}
