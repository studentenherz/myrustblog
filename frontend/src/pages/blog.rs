use log::info;
use std::rc::Rc;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew::Callback;

use crate::{components::PostCard, pages::Layout, services::api::ApiService, utils::set_title};

#[function_component(Blog)]
pub fn blog() -> Html {
    let page = use_state(|| 1u64);
    let pages = use_state(|| 1u64);
    let posts = use_state(|| vec![]);

    let update_posts = {
        let posts = posts.clone();
        let pages = pages.clone();

        Callback::from(move |page: u64| {
            let posts = posts.clone();
            let pages = pages.clone();

            spawn_local(async move {
                match ApiService::get_posts(Some(page), Some(10), None, None).await {
                    Ok((fetched_posts, total_pages)) => {
                        posts.set(fetched_posts.into_iter().map(Rc::new).collect());
                        pages.set(total_pages.unwrap_or(1));
                    }
                    Err(err) => {
                        info!("Error {:?}", err);
                    }
                }
            });
        })
    };

    {
        let update_posts = update_posts.clone();
        let page = *page;

        use_effect_with(page, move |page| {
            set_title("Blog");
            update_posts.emit(*page);
        });
    }

    let next_page = {
        let page = page.clone();
        let update_posts = update_posts.clone();
        Callback::from(move |_| {
            let new_page = *page + 1;
            page.set(new_page);
            update_posts.emit(new_page);
        })
    };

    let previous_page = {
        let page = page.clone();
        let update_posts = update_posts.clone();
        Callback::from(move |_| {
            let new_page = *page - 1;
            page.set(new_page);
            update_posts.emit(new_page);
        })
    };

    html! {
        <Layout>
            <div class="blog-posts-list">
                <div class="posts-container">
                    { for posts.iter().map(|post| html! {
                        <PostCard post={post.clone()} />
                    }) }
                </div>
                <div class="posts-container-navigation">
                    <button
                        class="prevent-default"
                        disabled={*page <= 1}
                        onclick={previous_page}>
                        <i class="fas fa-arrow-left icon"></i> { "Previous page" }
                    </button>
                    <div>{ *page } { " / " }  {*pages} </div>
                    <button
                        class="prevent-default"
                        disabled={*page >= *pages}
                        onclick={next_page}>
                        { "Next page" } <i class="fas fa-arrow-right icon"></i>
                    </button>
                </div>
            </div>
        </Layout>
    }
}
