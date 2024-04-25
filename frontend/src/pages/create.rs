use pulldown_cmark::{html::push_html, Options, Parser};
use wasm_bindgen_futures::spawn_local;
use web_sys::{wasm_bindgen::JsCast, HtmlInputElement, HtmlTextAreaElement};
use yew::prelude::*;
use yew_router::{history::History, prelude::*};

use crate::{
    pages::Layout,
    routes::AppRoute,
    services::api::{ApiError, ApiService},
    utils::{is_loged_in, set_title},
};

#[derive(Debug, Default)]
pub struct CreatePage {
    title: String,
    content: String,
    preview: bool,
}

pub enum Msg {
    UpdateTitle(String),
    UpdateContent(String),
    CreatePost,
    CreatePostError(ApiError),
    TogglePreview,
    Ignore,
}

impl Component for CreatePage {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self::default()
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if !is_loged_in() {
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
                            onclick={ctx.link().callback(|_| Msg::CreatePost)}
                        > { "Publish" } </button>
                    </div>
                    if self.preview {
                        <div class="md-preview">
                            { Html::from_html_unchecked(html_out.into()) }
                            </div>
                    }
                    else {
                        <div class="md-editor">
                            <input type="text" placeholder={"Write the title of your article..."}
                            value={self.title.clone()}
                            oninput={ctx.link().callback(|e: InputEvent| {
                                if let Some(input) = e.target().and_then(|t| t.dyn_into::<HtmlInputElement>().ok()) {
                                    Msg::UpdateTitle(input.value())
                                } else {
                                    Msg::Ignore
                                }
                            })}/>
                            <textarea  placeholder={"Write your article here using markdown..."}
                            value={self.content.clone()}
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
                self.content = content;
            }
            Msg::UpdateTitle(title) => {
                set_title(&format!("Creating | {}", title));
                self.title = title;
            }
            Msg::TogglePreview => {
                self.preview = !self.preview;
            }
            Msg::CreatePost => {
                let title = self.title.clone();
                let content = self.content.clone();

                let api_error_cb = ctx.link().callback(|err| Msg::CreatePostError(err));

                spawn_local(async move {
                    match ApiService::create_post(&title, &content).await {
                        Ok(slug) => {
                            yew_router::history::BrowserHistory::new()
                                .push(AppRoute::Post { slug }.to_path());
                        }
                        Err(err) => api_error_cb.emit(err),
                    }
                });
            }
            Msg::CreatePostError(err) => {
                log::error!("{:?}", err);
            }
        }

        true
    }
}
