use std::collections::HashMap;

use actix_web::{web, HttpResponse, Responder};

use crate::utils::Highlighter;
use common::CodeBlock;

pub async fn highlight_code(
    highlighter: web::Data<Highlighter>,
    post: web::Json<HashMap<String, CodeBlock>>,
) -> impl Responder {
    let mut highlighted = HashMap::<String, String>::new();
    for (id, CodeBlock { lang, code }) in post.iter() {
        highlighted.insert(
            id.clone(),
            format!(
                r#"<span class="language-tag">.{lang}</span><code class="language-{lang}">{}</code>"#,
                highlighter
                    .parse_html_with_class_style_with_code_extension(&code, &lang)
                    .unwrap_or(code.clone())
            ),
        );
    }
    HttpResponse::Ok().json(highlighted)
}
