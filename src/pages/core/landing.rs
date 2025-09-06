use crate::shared::ui::buttons;
use yew::prelude::*;
pub fn landing_page() -> Html {
    html! {
    <div>
        <div>
            <h3> {"Landing"} </h3>
            {buttons::create_github_button_animated()}

            {buttons::create_linked_in_button_animated()}
        </div>
    </div>
    }
}
