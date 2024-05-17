use yew::prelude::*;

use crate::{
    components::{Footer, Header},
    utils::User,
};

#[derive(Properties, PartialEq, Default)]
pub struct LayoutProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub user: Option<User>,
}

#[function_component(Layout)]
pub fn layout(props: &LayoutProps) -> Html {
    html! {
        <div class="layout">
            <Header user={ props.user.clone() } />

            <main class="center-content">
                { for props.children.iter() }
            </main>

            <Footer />
        </div>
    }
}
