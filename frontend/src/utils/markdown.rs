use std::collections::HashMap;

use pulldown_cmark::{html, CowStr, Event, HeadingLevel, Options, Parser, Tag, TagEnd};

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

    let parser = parser.iter().map(|event| {
        let id_map = &id_map;
        match event {
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
            _ => event.clone(),
        }
    });

    let mut html_string = String::new();
    html::push_html(&mut html_string, parser);

    (headers, html_string)
}
