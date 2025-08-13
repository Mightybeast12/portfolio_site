use crate::styles::layout::layout as styles;
use crate::utils::navbar::NavBar;
use yew::prelude::*;

#[function_component(Layout)]
pub fn layout(props: &Props) -> Html {
    html! {
        <div class={styles::layout_main()}>
            <NavBar />
            <main>
                { for props.children.iter() }
            </main>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
}
