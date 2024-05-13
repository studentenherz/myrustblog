use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::{window, HtmlDialogElement, HtmlElement};
use yew::prelude::*;
use yew_router::history::History;
use yew_router::prelude::*;
use yewdux::prelude::*;

use crate::utils::set_description_meta_tag;
use crate::{
    pages::Layout,
    routes::AppRoute,
    services::api::ApiService,
    utils::{parse_markdown, set_title, AppState, User},
};
use common::{utils::get_summary, Post};

#[derive(PartialEq, Properties)]
pub struct Props {
    pub slug: AttrValue,
}

#[function_component(PostPage)]
pub fn post_page(props: &Props) -> Html {
    let post = use_state(Post::default);
    let post_content = use_state(String::new);
    let headers = use_state(Vec::new);
    let user = use_selector(|state: &AppState| state.user.clone());

    let slug = props.slug.clone();

    // Fetch post data
    {
        let post = post.clone();
        let post_content = post_content.clone();
        let headers = headers.clone();
        let slug = slug.clone();

        use_effect_with(slug, move |slug| {
            let post = post.clone();
            let post_content = post_content.clone();
            let headers = headers.clone();
            let slug = slug.clone();

            spawn_local(async move {
                match ApiService::get_post(&slug).await {
                    Ok(Some(fetched_post)) => {
                        let summary = get_summary(&fetched_post.content, 200);
                        set_description_meta_tag(&summary);
                        set_title(&format!("Blog | {}", &fetched_post.title));
                        let (parsed_headers, parsed_content, codeblocks) =
                            parse_markdown(&fetched_post.content);
                        headers.set(parsed_headers);
                        post_content.set(parsed_content);
                        post.set(fetched_post);

                        // Highlight code
                        if !codeblocks.is_empty() {
                            match ApiService::highlight_code(codeblocks).await {
                                Ok(highlighted) => {
                                    if let Some(window) = window() {
                                        if let Some(document) = window.document() {
                                            for (id, code) in highlighted.iter() {
                                                if let Some(element) =
                                                    document.get_element_by_id(&id)
                                                {
                                                    if let Ok(html_element) =
                                                        element.dyn_into::<HtmlElement>()
                                                    {
                                                        html_element.set_inner_html(code);
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                Err(_) => {
                                    log::error!("Error highlighting code")
                                }
                            }
                        }
                    }
                    Ok(None) => yew_router::history::BrowserHistory::new()
                        .replace(AppRoute::NotFound.to_path()),
                    Err(_) => {
                        log::error!("Error fetching post")
                    }
                }
            });

            || ()
        });
    }

    let delete_post = {
        let slug = post.slug.clone();
        Callback::from(move |_| {
            let slug = slug.clone();
            spawn_local(async move {
                match ApiService::_delete_post(&slug).await {
                    Ok(_) => {
                        yew_router::history::BrowserHistory::new().push(AppRoute::Home.to_path());
                    }
                    Err(_) => {
                        log::error!("Error deleting post")
                    }
                }
            });
        })
    };

    html! {
        <Layout>
            <div class="post-title">
                <h1>{ &post.title }</h1>
                <div class="details">
                    <p>{ &post.author }</p>
                    <time datetime={post.published_at.to_rfc2822()}>
                        { post.published_at.format("%d %b %Y").to_string() }
                    </time>
                </div>
            </div>
            <div class="post-container">
                <div class="content-table">
                    <h2>{"Contents"}</h2>
                    <ul>
                        { for headers.iter().map(|header| html! {
                            <li class={format!("header-{:?}", header.level)}>
                                <a href={format!("#{}", header.id.clone())}>{ header.text.clone() }</a>
                            </li>
                        }) }
                    </ul>
                </div>
                <div class="post">
                    { Html::from_html_unchecked((*post_content).clone().into()) }
                </div>

                if let Some(User { username: _, role }) = &*user {
                    if role == "Admin" || role == "Editor" {
                        <div class="post-edit-bar">
                            <Link<AppRoute> classes="clickable" to={AppRoute::Edit { slug: post.slug.clone() }}>
                                <i class="fa-regular fa-pen-to-square icon"></i> { "Edit this post" }
                            </Link<AppRoute>>
                            <button class="clickable" onclick={Callback::from(|_| {
                                if let Some(window) = window() {
                                    if let Some(document) = window.document() {
                                        if let Some(element) = document.get_element_by_id("delete-dialog") {
                                            if let Ok(dialog_element) = element.dyn_into::<HtmlDialogElement>() {
                                                let _ = dialog_element.show_modal();
                                            }
                                        }
                                    }
                                }
                            })}>
                                <i class="fa-solid fa-trash icon"></i> { "Delete" }
                            </button>
                            <dialog id="delete-dialog">
                                { "Are you sure you want to delete this post?" }
                                <div>
                                    <button onclick={Callback::from(|_| {
                                        if let Some(window) = window() {
                                            if let Some(document) = window.document() {
                                                if let Some(element) = document.get_element_by_id("delete-dialog") {
                                                    if let Ok(dialog_element) = element.dyn_into::<HtmlDialogElement>() {
                                                        let _ = dialog_element.close();
                                                    }
                                                }
                                            }
                                        }
                                    })}>
                                        { "Cancel" }
                                    </button>
                                    <button id="delete-button" onclick={delete_post}> { "Accept" } </button>
                                </div>
                            </dialog>
                        </div>
                    }
                }
            </div>
        </Layout>
    }
}
