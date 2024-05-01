use syntect::html::{ClassStyle, ClassedHTMLGenerator};
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;

#[derive(Clone)]
pub struct Highlighter {
    ss: SyntaxSet,
}

impl Highlighter {
    pub fn new() -> Self {
        let ss = SyntaxSet::load_defaults_newlines();

        Self { ss }
    }

    pub fn parse_html_with_class_style_with_code_extension(
        &self,
        code_text: &str,
        ext: &str,
    ) -> Option<String> {
        if let Some(sr) = self.ss.find_syntax_by_extension(ext) {
            let mut html_generator =
                ClassedHTMLGenerator::new_with_class_style(sr, &self.ss, ClassStyle::Spaced);
            for line in LinesWithEndings::from(code_text) {
                if html_generator
                    .parse_html_for_line_which_includes_newline(line)
                    .is_err()
                {
                    return None;
                }
            }

            return Some(html_generator.finalize());
        }

        None
    }
}
