use pulldown_cmark::{html::push_html, Options, Parser};
use wasm_bindgen_futures::spawn_local;
use web_sys::{wasm_bindgen::JsCast, HtmlInputElement, HtmlTextAreaElement};
use yew::prelude::*;
use yew_router::{history::History, prelude::*};
use yewdux::Dispatch;

use crate::{
    pages::Layout,
    routes::AppRoute,
    services::api::{ApiError, ApiService},
    utils::{set_title, AppState},
};

#[derive(Debug, Default)]
struct EditPage {
    title: String,
    content: String,
    preview: bool,
    content_rows: u32,
}

enum Msg {
    UpdateTitle(String),
    UpdateContent(String),
    Publish,
    PublishError(ApiError),
    TogglePreview,
    Ignore,
}

#[derive(Properties, PartialEq)]
struct Props {
    #[prop_or_default]
    slug: Option<String>,
}

const CHARS_PER_LINE: u32 = 80;

impl Component for EditPage {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        if let Some(slug) = ctx.props().slug.clone() {
            let update_title_cb = ctx.link().callback(|title| Msg::UpdateTitle(title));
            let update_content_cb = ctx.link().callback(|content| Msg::UpdateContent(content));

            spawn_local(async move {
                match ApiService::get_post(&slug).await {
                    Ok(Some(post)) => {
                        set_title(&format!("Edit | {}", &post.title));
                        update_title_cb.emit(post.title);
                        update_content_cb.emit(post.content);
                    }
                    Ok(None) => yew_router::history::BrowserHistory::new()
                        .replace(AppRoute::NotFound.to_path()),
                    Err(_) => {
                        log::error!("Error")
                    }
                }
            });
        }

        Self::default()
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let dispatch = Dispatch::<AppState>::global();

        if dispatch.get().user.is_none() {
            return html! {
                <Redirect<AppRoute> to={AppRoute::Home}/>
            };
        }

        let parser = Parser::new_ext(
            &self.content,
            Options::ENABLE_TABLES | Options::ENABLE_TASKLISTS | Options::ENABLE_FOOTNOTES,
        );

        let mut html_out = String::new();
        push_html(&mut html_out, parser);

        html! {
            <Layout>
                <div class="post-title">
                    <input id="title-input" type="text" placeholder={"Title..."}
                    value={self.title.clone()}
                    oninput={ctx.link().callback(|e: InputEvent| {
                        if let Some(input) = e.target().and_then(|t| t.dyn_into::<HtmlInputElement>().ok()) {
                            Msg::UpdateTitle(input.value())
                        } else {
                            Msg::Ignore
                        }
                    })}/>
                </div>
                <div class="create-container">
                    <div class="editor-bar">
                    <div class="clickable" onclick={ctx.link().callback(|_e| Msg::TogglePreview)}>
                            if self.preview {
                                <i class="fas fa-pencil icon"></i> { "Edit" }
                            }
                            else{
                                <i class="fas fa-eye icon"></i> { "Preview" }
                            }
                            </div>
                        <button disabled={self.title.is_empty() || self.content.is_empty()}
                            onclick={ctx.link().callback(|_| Msg::Publish)}
                        > { "Publish" } </button>
                    </div>
                    if self.preview {
                        <div class="md-preview">
                            { Html::from_html_unchecked(html_out.into()) }
                            </div>
                    }
                    else {
                        <div class="md-editor">
                            <textarea  placeholder={"Write your article here using markdown..."}
                            value={self.content.clone()}
                            rows={self.content_rows.to_string()}
                            oninput={ctx.link().callback(|e: InputEvent| {
                                if let Some(input) = e.target().and_then(|t| t.dyn_into::<HtmlTextAreaElement>().ok()) {
                                    Msg::UpdateContent(input.value())
                                } else {
                                    Msg::Ignore
                                }
                            })}/>
                        </div>
                    }
                </div>
            </Layout>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Ignore => {}
            Msg::UpdateContent(content) => {
                self.content_rows = 2;
                for line in content.lines() {
                    self.content_rows += line.len() as u32 / CHARS_PER_LINE + 1;
                }
                self.content = content;
            }
            Msg::UpdateTitle(title) => {
                set_title(&format!("Creating | {}", title));
                self.title = title;
            }
            Msg::TogglePreview => {
                self.preview = !self.preview;
            }
            Msg::Publish => {
                let title = self.title.clone();
                let content = self.content.clone();
                let slug = ctx.props().slug.clone();

                let api_error_cb = ctx.link().callback(|err| Msg::PublishError(err));

                spawn_local(async move {
                    match if let Some(slug) = slug {
                        ApiService::_update_post(&slug, &content).await
                    } else {
                        ApiService::create_post(&title, &content).await
                    } {
                        Ok(slug) => {
                            yew_router::history::BrowserHistory::new()
                                .push(AppRoute::Post { slug }.to_path());
                        }
                        Err(err) => api_error_cb.emit(err),
                    }
                });
            }
            Msg::PublishError(err) => {
                log::error!("{:?}", err);
            }
        }

        true
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
