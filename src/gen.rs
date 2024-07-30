use gtmpl::{gtmpl_fn, Context, FuncError, Template, Value};
use gtmpl_derive::Gtmpl;
use markdown::{
    mdast::{Code, Node},
    unist::Point,
    Constructs, ParseOptions,
};

use crate::{language::Language, Error};

#[derive(Gtmpl, Clone)]
struct TmplContext {
    segments: Vec<SegmentContext>,
    include: Vec<String>,
}

#[derive(Gtmpl, Clone)]
struct SegmentContext {
    code: Option<String>,
    text: Option<String>,
    line: usize,
}

const CONSTRUCTS: Constructs = Constructs {
    attention: false,
    autolink: false,
    block_quote: false,
    character_escape: false,
    character_reference: false,
    code_indented: false,
    code_fenced: true,
    code_text: false,
    definition: false,
    frontmatter: false,
    gfm_autolink_literal: false,
    gfm_footnote_definition: false,
    gfm_label_start_footnote: false,
    gfm_strikethrough: false,
    gfm_table: false,
    gfm_task_list_item: false,
    hard_break_escape: false,
    hard_break_trailing: false,
    heading_atx: false,
    heading_setext: false,
    html_flow: false,
    html_text: false,
    label_start_image: false,
    label_start_link: false,
    label_end: false,
    list_item: false,
    math_flow: false,
    math_text: false,
    mdx_esm: false,
    mdx_expression_flow: false,
    mdx_expression_text: false,
    mdx_jsx_flow: false,
    mdx_jsx_text: false,
    thematic_break: false,
};

pub struct Gen {
    lang: &'static Language,
    gtmpl: Template,
}

gtmpl_fn! {
    fn tmpl_prepend(
        prefix: String,
        value: String
    ) -> Result<Value, FuncError> {
        Ok(value
            .lines()
            .flat_map(|line| [prefix.as_str(), line, "\n"])
            .collect::<String>()
            .into())
    }
}

impl Gen {
    pub fn new(lang: &'static Language) -> Result<Self, Error> {
        let mut gtmpl = Template::default();
        gtmpl.add_func("prepend", tmpl_prepend);
        gtmpl.parse(lang.template)?;
        Ok(Gen { lang, gtmpl })
    }

    pub fn generate(
        &self,
        input: &str,
        include: Vec<String>,
    ) -> Result<String, Error> {
        let root = markdown::to_mdast(
            input,
            &ParseOptions {
                constructs: CONSTRUCTS,
                ..Default::default()
            },
        )
        .map_err(|e| e.to_string())?;

        let segments = self.gen_segments(input, root.children().unwrap());
        Ok(self
            .gtmpl
            .render(&Context::from(TmplContext { segments, include }))?)
    }

    fn find_positions(
        &self,
        blocks: &[Node],
        positions: &mut Vec<(Option<Code>, Point)>,
    ) {
        for block in blocks {
            let position = block.position().unwrap().start.clone();
            if let Node::Code(code) = block {
                let code = if self.lang.has_name_code(code) {
                    Some(code.clone())
                } else {
                    None
                };
                positions.push((code, position));
            } else if let Some(children) = block.children() {
                self.find_positions(children, positions);
            } else if positions
                .last()
                .map_or(true, |(is_code, _)| is_code.is_some())
            {
                positions.push((None, position));
            }
        }
    }

    fn gen_segments(
        &self,
        input: &str,
        blocks: &[Node],
    ) -> Vec<SegmentContext> {
        let mut positions = Vec::new();
        self.find_positions(blocks, &mut positions);
        positions.push((None, Point::new(0, 0, input.chars().count())));
        let mut result = vec![];
        let mut last_position: Option<(Option<Code>, Point)> = None;
        for position in positions {
            match last_position {
                None => {}
                Some((Some(ref code), ref position)) => {
                    result.push(SegmentContext {
                        code: Some(code.value.clone()),
                        text: None,
                        line: position.line,
                    });
                }
                Some((None, ref last_position)) => {
                    let text = input
                        .chars()
                        .skip(last_position.offset)
                        .take(position.1.offset - last_position.offset)
                        .collect();
                    result.push(SegmentContext {
                        code: None,
                        text: Some(text),
                        line: last_position.line,
                    });
                }
            }
            last_position = Some(position);
        }
        result
    }
}
