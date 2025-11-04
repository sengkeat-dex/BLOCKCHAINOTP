//! Header component for the application

use yew::prelude::*;

/// Properties for the Header component
#[derive(Properties, PartialEq)]
pub struct HeaderProps {
    #[prop_or_default]
    pub title: String,
}

/// Header component
#[function_component(Header)]
pub fn header(props: &HeaderProps) -> Html {
    html! {
        <header class="app-header">
            <h1>{ &props.title }</h1>
            <p>{"Secure authentication with blockchain verification"}</p>
        </header>
    }
}
