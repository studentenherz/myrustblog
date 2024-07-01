use std::sync::Arc;

use yew::prelude::*;

use crate::utils::User;
use common::{Header, Post};

#[derive(PartialEq, Properties)]
pub struct PostProps {
    pub slug: AttrValue,
    #[prop_or_default]
    pub post: Arc<Post>,
    #[prop_or_default]
    pub post_content: AttrValue,
    #[prop_or_default]
    pub headers: Vec<Header>,
    #[prop_or_default]
    pub user: Option<User>,
}

#[function_component(PostPage)]
pub fn post_page(
    PostProps {
        slug,
        post,
        post_content,
        headers,
        user,
    }: &PostProps,
) -> Html {
    html! {
        <>
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
                            <li id={format!("ct-{}", header.id.clone())} class={format!("header-{:?}", header.level)}>
                                <a href={format!("#{}", header.id.clone())}>{ Html::from_html_unchecked(header.text.clone().into()) }</a>
                            </li>
                        }) }
                    </ul>
                </div>
                <div class="post">
                    { Html::from_html_unchecked((*post_content).clone().into()) }
                </div>

                if let Some(User { username: _, role }) = user {
                    if role == "Admin" || role == "Editor" {
                        <div class="post-edit-bar">
                            <a class="clickable" href={ format!("/edit/{}", post.slug.clone()) }>
                                <i class="icon-edit icon"></i> { "Edit this post" }
                            </a>
                            {
                                Html::from_html_unchecked(r#"
                                <button onclick="document.getElementById('delete-dialog').showModal()">
                                    <i class="icon-trash icon"></i> Delete
                                </button>
                                "#.into())
                            }

                            <dialog id="delete-dialog">
                                { "Are you sure you want to delete this post?" }
                                <div>
                                    {
                                        Html::from_html_unchecked(r#"
                                        <button onclick="document.getElementById('delete-dialog').close()">
                                            Cancel
                                        </button>
                                        "#.into())
                                    }
                                    <a id="delete-button" class="button" href={ format!("/delete/{}", slug) }>
                                        { "Accept" }
                                    </a>
                                </div>
                            </dialog>
                        </div>
                    }
                }
            </div>
        </>
    }
}
