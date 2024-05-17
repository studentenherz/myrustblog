use pulldown_cmark::{html::push_html, Options, Parser};
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlInputElement, HtmlTextAreaElement};
use yew::prelude::*;
use yew_router::history::History;

use crate::{
    pages::Layout,
    services::api::{ApiError, ApiService},
    utils::set_title,
};

#[derive(Properties, PartialEq)]
struct Props {
    #[prop_or_default]
    slug: Option<String>,
}

const CHARS_PER_LINE: u32 = 80;

#[function_component(EditPage)]
fn edit_page(props: &Props) -> Html {
    let title = use_state(String::new);
    let content = use_state(String::new);
    let preview = use_state(|| false);
    let content_rows = use_state(|| 2);

    let slug = props.slug.clone();

    {
        let title = title.clone();
        let content = content.clone();
        let slug = slug.clone();

        use_effect_with(slug, move |slug| {
            let title = title.clone();
            let content = content.clone();

            if let Some(slug) = slug.clone() {
                spawn_local(async move {
                    match ApiService::get_post(&slug).await {
                        Ok(Some(post)) => {
                            set_title(&format!("Edit | {}", &post.title));
                            title.set(post.title);
                            content.set(post.content);
                        }
                        Ok(None) => yew_router::history::BrowserHistory::new().replace("/404"),
                        Err(_) => {
                            log::error!("Error fetching post")
                        }
                    }
                });
            }

            || ()
        });
    }

    let on_update_title = {
        let title = title.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                title.set(input.value());
            }
        })
    };

    let on_editor_input = {
        let content = content.clone();
        Callback::from(move |e: InputEvent| {
            log::info!("{:?}", e);

            if let Some(input) = e.target_dyn_into::<HtmlTextAreaElement>() {
                let content_value = input.value();
                content.set(content_value);
            }
        })
    };

    let onkeydown = {
        let content = content.clone();
        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Tab" {
                e.prevent_default();

                if let Some(target) = e.target_dyn_into::<HtmlTextAreaElement>() {
                    let value = target.value();
                    let start = target.selection_start().unwrap_or(Some(0)).unwrap_or(0) as usize;
                    let end = target.selection_end().unwrap_or(Some(0)).unwrap_or(0) as usize;

                    let new_value = format!("{}{}{}", &value[..start], "\t", &value[end..]);
                    content.set(new_value.clone());

                    target.set_value(&new_value);
                    target
                        .set_selection_start(Some((start + 1) as u32))
                        .unwrap();
                    target.set_selection_end(Some((start + 1) as u32)).unwrap();
                }
            }
        })
    };

    {
        let content = content.clone();
        let content_rows = content_rows.clone();

        use_effect_with(content, move |content| {
            let rows = content.lines().fold(2, |acc, line| {
                acc + (line.len() as u32 / CHARS_PER_LINE + 1)
            });
            content_rows.set(rows);
        })
    }

    let on_toggle_preview = {
        let preview = preview.clone();
        Callback::from(move |_| {
            preview.set(!*preview);
        })
    };

    let on_publish = {
        let title = title.clone();
        let content = content.clone();
        let slug = props.slug.clone();

        Callback::from(move |_| {
            let title = title.clone();
            let content = content.clone();
            let api_error_cb = Callback::from(|err: ApiError| log::error!("{:?}", err));
            let slug = slug.clone();

            spawn_local(async move {
                match if let Some(slug) = slug {
                    ApiService::_update_post(&slug, &content, &title).await
                } else {
                    ApiService::create_post(&title, &content).await
                } {
                    Ok(slug) => {
                        if let Some(window) = web_sys::window() {
                            let _ = window.location().replace(&format!("/post/{}", slug));
                        }
                    }
                    Err(err) => api_error_cb.emit(err),
                }
            });
        })
    };

    let parser = Parser::new_ext(
        &content,
        Options::ENABLE_TABLES | Options::ENABLE_TASKLISTS | Options::ENABLE_FOOTNOTES,
    );

    let mut html_out = String::new();
    push_html(&mut html_out, parser);

    html! {
        <Layout>
            <div class="post-title">
                <input id="title-input" type="text" placeholder={"Title..."}
                value={(*title).clone()}
                oninput={on_update_title}/>
            </div>
            <div class="create-container">
                <div class="editor-bar">
                    <div class="clickable" onclick={on_toggle_preview}>
                        if *preview {
                            <i class="icon-pencil icon"></i> { "Edit" }
                        } else {
                            <i class="icon-eye icon"></i> { "Preview" }
                        }
                    </div>
                    <button disabled={title.is_empty() || content.is_empty()}
                        onclick={on_publish}>
                        { "Publish" }
                    </button>
                </div>
                if *preview {
                    <div class="md-preview">
                        { Html::from_html_unchecked(html_out.into()) }
                    </div>
                } else {
                    <div class="md-editor">
                        <textarea placeholder={"Write your article here using markdown..."}
                        value={(*content).clone()}
                        rows={content_rows.to_string()}
                        oninput={on_editor_input}
                        onkeydown={onkeydown}/>
                    </div>
                }
            </div>
        </Layout>
    }
}

#[function_component(CreatePost)]
pub fn create_post() -> Html {
    html! {
        <EditPage />
    }
}

#[derive(Properties, PartialEq)]
pub struct EditProps {
    pub slug: String,
}

#[function_component(EditPost)]
pub fn edit_post(EditProps { slug }: &EditProps) -> Html {
    html! {
        <EditPage slug={Some(slug.clone())} />
    }
}
