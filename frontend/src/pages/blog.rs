use log::info;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::{components::PostCard, pages::Layout, routes::AppRoute, services::api::ApiService};

use common::Post;

#[derive(Debug, Default)]
pub struct Blog {
    pub page: u64,
    pub posts: Vec<Post>,
}

pub enum Msg {
    UpdatePosts(Vec<Post>),
}

impl Component for Blog {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let update_posts_cb = ctx
            .link()
            .callback(move |posts: Vec<Post>| Msg::UpdatePosts(posts));

        spawn_local(async move {
            match ApiService::get_posts(Some(1), Some(10), None, None).await {
                Ok(posts) => {
                    update_posts_cb.emit(posts);
                }
                Err(err) => {
                    info!("Error {:?}", err);
                }
            }
        });

        Self {
            page: 1,
            ..Self::default()
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <Layout>
                <div class="posts-container">
                    { for self.posts.iter().map(|post| html! {
                        <PostCard
                            title={AttrValue::from(post.title.clone())}
                            author={AttrValue::from(post.author.clone())}
                            slug={AttrValue::from(post.slug.clone())}
                        />
                    } ) }
                </div>
            </Layout>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdatePosts(posts) => {
                self.posts = posts;
            }
        }

        true
    }
}
