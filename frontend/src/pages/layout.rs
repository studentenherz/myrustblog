use yew::prelude::*;

use crate::components::{Footer, Header};

#[derive(Properties, PartialEq)]
pub struct LayoutProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Layout)]
pub fn layout(props: &LayoutProps) -> Html {
    html! {
        <div class="layout">
            <Header />

            <main>
                { for props.children.iter() }
            </main>

            <Footer />
        </div>
    }
}
