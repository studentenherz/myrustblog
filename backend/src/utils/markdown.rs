use std::collections::HashMap;

use pulldown_cmark::{
    html, BlockQuoteKind, CodeBlockKind, CowStr, Event, HeadingLevel, Options, Parser, Tag, TagEnd,
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
        Event::Start(Tag::BlockQuote(Some(kind))) => {
            let quote_heading = match kind {
                BlockQuoteKind::Note => r#"<p class="markdown-alert-title" dir="auto"><svg class="octicon octicon-info mr-2" viewBox="0 0 16 16" version="1.1" width="16" height="16" aria-hidden="true"><path d="M0 8a8 8 0 1 1 16 0A8 8 0 0 1 0 8Zm8-6.5a6.5 6.5 0 1 0 0 13 6.5 6.5 0 0 0 0-13ZM6.5 7.75A.75.75 0 0 1 7.25 7h1a.75.75 0 0 1 .75.75v2.75h.25a.75.75 0 0 1 0 1.5h-2a.75.75 0 0 1 0-1.5h.25v-2h-.25a.75.75 0 0 1-.75-.75ZM8 6a1 1 0 1 1 0-2 1 1 0 0 1 0 2Z"></path></svg>Note</p>"#,
                BlockQuoteKind::Tip => r#"<p class="markdown-alert-title" dir="auto"><svg class="octicon octicon-light-bulb mr-2" viewBox="0 0 16 16" version="1.1" width="16" height="16" aria-hidden="true"><path d="M8 1.5c-2.363 0-4 1.69-4 3.75 0 .984.424 1.625.984 2.304l.214.253c.223.264.47.556.673.848.284.411.537.896.621 1.49a.75.75 0 0 1-1.484.211c-.04-.282-.163-.547-.37-.847a8.456 8.456 0 0 0-.542-.68c-.084-.1-.173-.205-.268-.32C3.201 7.75 2.5 6.766 2.5 5.25 2.5 2.31 4.863 0 8 0s5.5 2.31 5.5 5.25c0 1.516-.701 2.5-1.328 3.259-.095.115-.184.22-.268.319-.207.245-.383.453-.541.681-.208.3-.33.565-.37.847a.751.751 0 0 1-1.485-.212c.084-.593.337-1.078.621-1.489.203-.292.45-.584.673-.848.075-.088.147-.173.213-.253.561-.679.985-1.32.985-2.304 0-2.06-1.637-3.75-4-3.75ZM5.75 12h4.5a.75.75 0 0 1 0 1.5h-4.5a.75.75 0 0 1 0-1.5ZM6 15.25a.75.75 0 0 1 .75-.75h2.5a.75.75 0 0 1 0 1.5h-2.5a.75.75 0 0 1-.75-.75Z"></path></svg>Tip</p>"#,
                BlockQuoteKind::Important => r#"<p class="markdown-alert-title" dir="auto"><svg class="octicon octicon-report mr-2" viewBox="0 0 16 16" version="1.1" width="16" height="16" aria-hidden="true"><path d="M0 1.75C0 .784.784 0 1.75 0h12.5C15.216 0 16 .784 16 1.75v9.5A1.75 1.75 0 0 1 14.25 13H8.06l-2.573 2.573A1.458 1.458 0 0 1 3 14.543V13H1.75A1.75 1.75 0 0 1 0 11.25Zm1.75-.25a.25.25 0 0 0-.25.25v9.5c0 .138.112.25.25.25h2a.75.75 0 0 1 .75.75v2.19l2.72-2.72a.749.749 0 0 1 .53-.22h6.5a.25.25 0 0 0 .25-.25v-9.5a.25.25 0 0 0-.25-.25Zm7 2.25v2.5a.75.75 0 0 1-1.5 0v-2.5a.75.75 0 0 1 1.5 0ZM9 9a1 1 0 1 1-2 0 1 1 0 0 1 2 0Z"></path></svg>Important</p>"#,
                BlockQuoteKind::Warning => r#"<p class="markdown-alert-title" dir="auto"><svg class="octicon octicon-alert mr-2" viewBox="0 0 16 16" version="1.1" width="16" height="16" aria-hidden="true"><path d="M6.457 1.047c.659-1.234 2.427-1.234 3.086 0l6.082 11.378A1.75 1.75 0 0 1 14.082 15H1.918a1.75 1.75 0 0 1-1.543-2.575Zm1.763.707a.25.25 0 0 0-.44 0L1.698 13.132a.25.25 0 0 0 .22.368h12.164a.25.25 0 0 0 .22-.368Zm.53 3.996v2.5a.75.75 0 0 1-1.5 0v-2.5a.75.75 0 0 1 1.5 0ZM9 11a1 1 0 1 1-2 0 1 1 0 0 1 2 0Z"></path></svg>Warning</p>"#,
                BlockQuoteKind::Caution => r#"<p class="markdown-alert-title" dir="auto"><svg class="octicon octicon-stop mr-2" viewBox="0 0 16 16" version="1.1" width="16" height="16" aria-hidden="true"><path d="M4.47.22A.749.749 0 0 1 5 0h6c.199 0 .389.079.53.22l4.25 4.25c.141.14.22.331.22.53v6a.749.749 0 0 1-.22.53l-4.25 4.25A.749.749 0 0 1 11 16H5a.749.749 0 0 1-.53-.22L.22 11.53A.749.749 0 0 1 0 11V5c0-.199.079-.389.22-.53Zm.84 1.28L1.5 5.31v5.38l3.81 3.81h5.38l3.81-3.81V5.31L10.69 1.5ZM8 4a.75.75 0 0 1 .75.75v3.5a.75.75 0 0 1-1.5 0v-3.5A.75.75 0 0 1 8 4Zm0 8a1 1 0 1 1 0-2 1 1 0 0 1 0 2Z"></path></svg>Caution</p>"#,
            };

            parser.push(event);
            parser.push(Event::Html(CowStr::Borrowed(quote_heading)))
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
