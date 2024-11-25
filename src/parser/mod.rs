use crate::utils;
pub use tree_sitter::{Language, Parser, Tree};

pub fn get_grammar_pairs() -> Vec<(&'static str, Language, Vec<&'static str>)> {
    vec![
        ("Java", tree_sitter_java::LANGUAGE.into(), vec![".java"]),
        ("Python", tree_sitter_python::LANGUAGE.into(), vec![".py"]),
    ]
}

pub struct TSParser {
    name: &'static str,
    parser: Parser,
    supported_extensions: Vec<&'static str>,
}

impl TSParser {
    pub fn new(name: &'static str, grammar: Language) -> Self {
        let mut parser = Parser::new();
        parser
            .set_language(&grammar)
            .expect("Error setting language");
        Self {
            name,
            parser,
            supported_extensions: vec![],
        }
    }
}

pub struct TSParsers {
    ts_parsers: Vec<TSParser>,
}

impl TSParsers {
    pub fn new() -> Self {
        let ts_parsers = get_grammar_pairs()
            .into_iter()
            .map(|(name, grammar, extensions)| {
                let mut parser = TSParser::new(name, grammar);
                parser.supported_extensions = extensions;
                parser
            })
            .collect();
        Self { ts_parsers }
    }

    pub fn generate_tree(&mut self, file_path: &str) -> Option<Tree> {
        let file_extension = utils::get_file_extension(file_path);

        for ts_parser in &mut self.ts_parsers {
            if ts_parser
                .supported_extensions
                .contains(&file_extension.as_str())
            {
                let source_code = utils::read_file(file_path);
                return ts_parser.parser.parse(source_code, None);
            }
        }
        None
    }
}
