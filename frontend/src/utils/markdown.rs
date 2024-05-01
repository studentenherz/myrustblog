use std::{cmp::min, collections::HashMap};

use log::info;
use pulldown_cmark::{
    html, CodeBlockKind, CowStr, Event, HeadingLevel, Options, Parser, Tag, TagEnd,
};
use syntect::html::{ClassStyle, ClassedHTMLGenerator};
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;

use common::utils::title_to_slug;

#[derive(Debug, PartialEq)]
pub struct Header {
    pub level: HeadingLevel,
    pub text: String,
    pub id: String,
}

pub fn get_headers_and_html_with_ids(html_text: &str) -> (Vec<Header>, String) {
    let parser = Parser::new_ext(
        html_text,
        Options::ENABLE_TABLES | Options::ENABLE_TASKLISTS | Options::ENABLE_FOOTNOTES,
    );

    let ss = SyntaxSet::load_defaults_newlines();

    let mut headers: Vec<Header> = vec![];
    let mut in_header = false;
    let mut header_level: HeadingLevel = HeadingLevel::H1;
    let mut header_text = String::new();
    let mut idn = 0usize;
    let mut id_map = HashMap::<CowStr, CowStr>::new();
    let id_prefix = "heading-id312";

    let parser: Vec<Event> = parser
        .map(|event| match event {
            Event::Start(Tag::Heading {
                level,
                id,
                classes,
                attrs,
            }) => {
                in_header = true;
                header_level = level;
                header_text = String::new();

                let id = if id.is_some() {
                    id
                } else {
                    idn += 1;
                    Some(format!("{}-{}", id_prefix, idn).into())
                };

                Event::Start(Tag::Heading {
                    level,
                    id,
                    classes,
                    attrs,
                })
            }
            Event::End(TagEnd::Heading(_)) => {
                in_header = false;

                let id = title_to_slug(&header_text);
                id_map.insert(format!("{}-{}", id_prefix, idn).into(), id.clone().into());

                headers.push(Header {
                    level: header_level,
                    text: header_text.clone(),
                    id,
                });

                event
            }
            Event::Text(text) if in_header => {
                header_text.push_str(&text);
                Event::Text(text)
            }
            _ => event,
        })
        .collect();

    let mut in_codeblock = false;
    let mut lang = "";
    let parser = parser.iter().map(|event| match event {
        Event::Start(Tag::Heading {
            level,
            id,
            classes,
            attrs,
        }) => {
            let id = id
                .clone()
                .map(|id_val| id_map.get(&id_val).unwrap_or(&id_val).clone());

            Event::Start(Tag::Heading {
                level: *level,
                id,
                classes: classes.clone(),
                attrs: attrs.clone(),
            })
        }
        Event::Start(Tag::CodeBlock(cb)) => {
            in_codeblock = true;
            lang = match cb {
                CodeBlockKind::Indented => "",
                CodeBlockKind::Fenced(lng) => lng,
            };
            event.clone()
        }
        Event::Text(code_text) if in_codeblock => {
            if let Some(sr_rs) = ss.find_syntax_by_extension(lang) {
                let mut rs_html_generator =
                    ClassedHTMLGenerator::new_with_class_style(sr_rs, &ss, ClassStyle::Spaced);
                for line in LinesWithEndings::from(code_text) {
                    rs_html_generator
                        .parse_html_for_line_which_includes_newline(line)
                        .unwrap();
                }
                let html_rs = rs_html_generator.finalize();

                Event::Html(html_rs.into())
            } else {
                event.clone()
            }
        }
        Event::End(TagEnd::CodeBlock) => {
            in_codeblock = false;
            event.clone()
        }
        _ => event.clone(),
    });

    let mut html_string = String::new();
    html::push_html(&mut html_string, parser);

    (headers, html_string)
}

pub fn get_summary(html_text: &str, max_len: usize) -> String {
    let mut summary = String::new();

    let parser = Parser::new_ext(
        html_text,
        Options::ENABLE_TABLES | Options::ENABLE_TASKLISTS | Options::ENABLE_FOOTNOTES,
    );
    let mut in_p = false;

    for event in parser {
        match event {
            Event::Start(Tag::Paragraph) => {
                in_p = true;
            }
            Event::End(TagEnd::Paragraph) => {
                in_p = false;
            }
            Event::Text(text) if in_p => {
                summary += &text[..min(max_len - summary.len(), text.len())];
                if summary.len() >= max_len {
                    break;
                }
            }
            _ => {}
        }
    }

    summary
}
