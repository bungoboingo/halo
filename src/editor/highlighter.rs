use iced::advanced::text::highlighter::Format;
use iced::{Color, Font};
use once_cell::sync::Lazy;
use std::ops::Range;
use syntect::highlighting::StyleModifier;
use syntect::parsing;

static WGSL_SYNTAX: Lazy<parsing::SyntaxSet> = Lazy::new(|| {
    parsing::SyntaxSet::load_from_folder(format!("{}/assets", env!("CARGO_MANIFEST_DIR")))
        .expect("Couldn't load WGSL syntax set")
});

static THEMES: Lazy<syntect::highlighting::ThemeSet> =
    Lazy::new(syntect::highlighting::ThemeSet::load_defaults);

const LINES_PER_SNAPSHOT: usize = 50;

pub struct Highlighter {
    syntax: &'static parsing::SyntaxReference,
    highlighter: syntect::highlighting::Highlighter<'static>,
    //TODO wut
    caches: Vec<(parsing::ParseState, parsing::ScopeStack)>,
    current_line: usize,
    errors: Vec<Range<usize>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Settings {
    pub theme: iced::highlighter::Theme,
    pub errors: Vec<Range<usize>>,
}

pub struct Highlight(StyleModifier);

impl Highlight {
    pub fn color(&self) -> Option<Color> {
        self.0
            .foreground
            .map(|color| Color::from_rgba8(color.r, color.g, color.b, color.a as f32 / 255.0))
    }

    pub fn font(&self) -> Option<Font> {
        None
    }

    pub fn to_format(&self) -> Format<Font> {
        Format {
            color: self.color(),
            font: self.font(),
        }
    }
}

impl iced::advanced::text::Highlighter for Highlighter {
    type Settings = Settings;
    type Highlight = Highlight;

    //TODO
    type Iterator<'a> = Box<dyn Iterator<Item = (Range<usize>, Self::Highlight)> + 'a>;

    fn new(settings: &Self::Settings) -> Self {
        let syntax = WGSL_SYNTAX
            .find_syntax_by_extension("wgsl")
            .unwrap_or_else(|| WGSL_SYNTAX.find_syntax_plain_text());

        let highlighter =
            syntect::highlighting::Highlighter::new(&THEMES.themes["base16-mocha.dark"]);

        let parser = parsing::ParseState::new(syntax);
        let stack = parsing::ScopeStack::new();

        Self {
            syntax,
            highlighter,
            caches: vec![(parser, stack)],
            current_line: 0,
            errors: settings.errors.clone(),
        }
    }

    fn update(&mut self, new_settings: &Self::Settings) {
        self.errors = new_settings.errors.clone();
        self.current_line = 0;
    }

    //TODO review
    fn change_line(&mut self, line: usize) {
        let snapshot = line / LINES_PER_SNAPSHOT;

        if snapshot <= self.caches.len() {
            self.caches.truncate(snapshot);
            self.current_line = snapshot * LINES_PER_SNAPSHOT;
        } else {
            self.caches.truncate(1);
            self.current_line = 0;
        }

        let (parser, stack) = self.caches.last().cloned().unwrap_or_else(|| {
            (
                parsing::ParseState::new(self.syntax),
                parsing::ScopeStack::new(),
            )
        });

        // Each line has its own parser & scope..?
        self.caches.push((parser, stack));
    }

    fn highlight_line(&mut self, line: &str) -> Self::Iterator<'_> {
        if self.current_line / LINES_PER_SNAPSHOT >= self.caches.len() {
            let (parser, stack) = self.caches.last().expect("Caches must not be empty");

            self.caches.push((parser.clone(), stack.clone()));
        }

        self.current_line += 1;

        let (parser, stack) = self.caches.last_mut().expect("Caches must not be empty");

        //parse, the single line, returns scope stack operation
        let ops = parser.parse_line(line, &WGSL_SYNTAX).unwrap_or_default();

        let highlighter = &self.highlighter;

        Box::new(
            ScopeRangeIterator {
                ops,
                line_length: line.len(),
                index: 0,
                last_str_index: 0,
            }
            .filter_map(move |(index, range, scope)| {
                let _ = stack.apply(&scope);

                if range.is_empty() {
                    None
                } else {
                    let modifier = highlighter.style_mod_for_stack(&stack.scopes);

                    Some((
                        range,
                        Highlight(highlighter.style_mod_for_stack(&stack.scopes)),
                    ))
                }
            }),
        )
    }

    fn current_line(&self) -> usize {
        self.current_line
    }
}

pub struct ScopeRangeIterator {
    ops: Vec<(usize, parsing::ScopeStackOp)>,
    line_length: usize,
    index: usize,
    last_str_index: usize,
}

impl Iterator for ScopeRangeIterator {
    type Item = (usize, Range<usize>, parsing::ScopeStackOp);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index > self.ops.len() {
            return None;
        }

        let next_str_i = if self.index == self.ops.len() {
            self.line_length
        } else {
            self.ops[self.index].0
        };

        let range = self.last_str_index..next_str_i;
        self.last_str_index = next_str_i;

        let op = if self.index == 0 {
            parsing::ScopeStackOp::Noop
        } else {
            self.ops[self.index - 1].1.clone()
        };

        self.index += 1;
        Some((self.index, range, op))
    }
}
