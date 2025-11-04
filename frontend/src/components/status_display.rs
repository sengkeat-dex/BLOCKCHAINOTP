//! Status Display component

use crate::utils::format_timestamp;
use yew::prelude::*;

/// Properties for the Status Display component
#[derive(Properties, PartialEq)]
pub struct StatusDisplayProps {
    pub request_id: String,
    pub expires_at: Option<u64>,
    pub message: String,
    pub verified: bool,
}

/// Status Display component
#[function_component(StatusDisplay)]
pub fn status_display(props: &StatusDisplayProps) -> Html {
    let request_info = if !props.request_id.is_empty() {
        html! {
            <div class="request-info">
                <p><strong>{"Request ID: "}</strong> { &props.request_id }</p>
                if let Some(expires_at) = props.expires_at {
                    <p><strong>{"Expires at: "}</strong> { format_timestamp(expires_at) }</p>
                }
            </div>
        }
    } else {
        html! {}
    };

    let status_class = if props.verified {
        "status-message success"
    } else if !props.message.is_empty() && !props.message.contains("successfully") {
        "status-message error"
    } else {
        "status-message info"
    };

    html! {
        <div class="status-display">
            { request_info }
            <div class={status_class}>
                { &props.message }
            </div>
        </div>
    }
}
