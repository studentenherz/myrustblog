use std::collections::HashMap;

use pulldown_cmark::{
    html, CodeBlockKind, CowStr, Event, HeadingLevel, Options, Parser, Tag, TagEnd,
};

use crate::utils::Highlighter;
use common::{utils::title_to_slug, Header};

pub fn parse_markdown(html_text: &str, highlighter: &Highlighter) -> (Vec<Header>, String) {
    let mut headers: Vec<Header> = vec![];
    let mut in_header = false;
    let mut header_level: HeadingLevel = HeadingLevel::H1;
    let mut header_plain_text = String::new();
    let mut idn = 0usize;
    let mut id_map = HashMap::<CowStr, CowStr>::new();
    let id_prefix = "heading-id312";
    let mut header_content: Vec<Event> = vec![];

    let mut section_headers_sack: Vec<HeadingLevel> = vec![];

    let mut parser: Vec<Event> = vec![];
    Parser::new_ext(
        html_text,
        Options::ENABLE_TABLES
            | Options::ENABLE_TASKLISTS
            | Options::ENABLE_FOOTNOTES
            | Options::ENABLE_MATH
            | Options::ENABLE_GFM,
    )
    .for_each(|event| match event {
        Event::Start(Tag::Heading {
            level,
            id,
            classes,
            attrs,
        }) => {
            in_header = true;
            header_level = level;
            header_plain_text = String::new();
            header_content.clear();

            let id = if id.is_some() {
                id
            } else {
                idn += 1;
                Some(format!("{}-{}", id_prefix, idn).into())
            };

            let mut section_enclose = String::new();

            while let Some(last_level) = section_headers_sack.last() {
                if header_level <= *last_level {
                    section_enclose += "</section>\n"; // Close section
                    section_headers_sack.pop();
                } else {
                    break;
                }
            }
            section_headers_sack.push(header_level);

            section_enclose += &format!("<section id={}>\n", id.clone().unwrap_or("id_err".into()));
            parser.push(Event::Html(section_enclose.into()));

            parser.push(Event::Start(Tag::Heading {
                level,
                id: None,
                classes,
                attrs,
            }))
        }
        Event::End(TagEnd::Heading(_)) => {
            in_header = false;

            let mut header_text = String::new();
            html::push_html(&mut header_text, header_content.clone().into_iter());

            let id = title_to_slug(&header_plain_text);
            id_map.insert(format!("{}-{}", id_prefix, idn).into(), id.clone().into());

            headers.push(Header {
                level: header_level,
                text: header_text.clone(),
                id: id.clone(),
            });

            parser.push(event)
        }
        Event::Text(ref text) if in_header => {
            header_plain_text.push_str(text);
            parser.push(event.clone());
            header_content.push(event);
        }
        Event::InlineMath(ref tex) => {
            let new_event = if let Ok(parsed) = katex::render(tex) {
                Event::Html(parsed.clone().into())
            } else {
                event
            };

            if in_header {
                header_content.push(new_event.clone());
            }

            parser.push(new_event);
        }
        Event::DisplayMath(ref tex) => {
            let opts = katex::Opts::builder().display_mode(true).build().unwrap();
            let new_event = if let Ok(parsed) = katex::render_with_opts(tex, opts) {
                Event::Html(parsed.into())
            } else {
                event
            };

            if in_header {
                header_content.push(new_event.clone());
            }

            parser.push(new_event);
        }
        _ if in_header => {
            header_content.push(event.clone());
            parser.push(event);
        }
        _ => parser.push(event),
    });

    let mut in_codeblock = false;
    let mut lang = "";
    let mut code_cum = String::new();
    let parser = parser.iter().filter_map(|event| match event {
        Event::Start(Tag::CodeBlock(cb)) => {
            in_codeblock = true;
            lang = match cb {
                CodeBlockKind::Indented => "",
                CodeBlockKind::Fenced(lng) => lng,
            };
            None
        }
        Event::Text(code_text) if in_codeblock => {
            code_cum.push_str(code_text);
            None
        }
        Event::End(TagEnd::CodeBlock) => {
            in_codeblock = false;

            let highlighted_code =
                highlighter.parse_html_with_class_style_with_code_extension(&code_cum, lang);

            let code_in_html_event = Some(Event::Html(
                format!(
                    r#"<pre><span class="language-tag">.{lang}</span><code class="language-{lang}">{}</code></pre>"#,
                    highlighted_code.unwrap_or(code_cum.clone())
                )
                .into(),
            ));

            code_cum.clear();
            code_in_html_event
        }
        _ => Some(event.clone()),
    });

    let mut html_string = String::new();
    html::push_html(&mut html_string, parser);

    id_map.into_iter().for_each(|(from, to)| {
        html_string = html_string.replacen(from.as_ref(), &to, 1);
    });

    for _ in 0..section_headers_sack.len() {
        html_string += "</section>\n";
    }

    (headers, html_string)
}
