use log::info;
use std::rc::Rc;

use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::{components::PostCard, pages::Layout, services::api::ApiService, utils::set_title};

use common::Post;

#[derive(Debug, Default)]
pub struct Blog {
    pub page: u64,
    pub pages: u64,
    pub posts: Vec<Rc<Post>>,
}

pub enum Msg {
    UpdatePosts((Vec<Post>, u64)),
    NextPage,
    PreviousPage,
}

impl Blog {
    fn update_posts(page: u64, ctx: &Context<Self>) {
        let update_posts_cb = ctx.link().callback(Msg::UpdatePosts);

        spawn_local(async move {
            match ApiService::get_posts(Some(page), Some(10), None, None).await {
                Ok((posts, pages)) => {
                    update_posts_cb.emit((posts, pages.unwrap_or(1)));
                }
                Err(err) => {
                    info!("Error {:?}", err);
                }
            }
        });
    }
}

impl Component for Blog {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self::update_posts(1, ctx);

        set_title("Blog");

        Self {
            page: 1,
            ..Self::default()
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <Layout>
                <div class="blog-posts-list">
                    <div class="posts-container">
                        { for self.posts.iter().map(|post| html! {
                            <PostCard
                                post = {post.clone()}
                            />
                        } ) }
                    </div>
                    <div class="posts-container-navigation">
                        <button
                            class="prevent-default"
                            disabled={self.page <= 1}
                            onclick={ctx.link().callback(|_|  Msg::PreviousPage)}>
                                <i class="fas fa-arrow-left icon"></i> { "Previous page" }
                        </button>
                        <div>{ self.page } { " / " }  {self.pages} </div>
                        <button
                            class="prevent-default"
                            disabled={self.page >= self.pages}
                            onclick={ctx.link().callback(|_|  Msg::NextPage)}>
                                { "Next page" } <i class="fas fa-arrow-right icon"></i>
                        </button>
                    </div>
                </div>
            </Layout>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdatePosts((posts, pages)) => {
                self.posts = posts.iter().map(|post| Rc::new(post.clone())).collect();
                self.pages = pages;
                return true;
            }
            Msg::NextPage => {
                self.page += 1;
                Self::update_posts(self.page, ctx);
            }
            Msg::PreviousPage => {
                self.page -= 1;
                Self::update_posts(self.page, ctx);
            }
        }

        false
    }
}
