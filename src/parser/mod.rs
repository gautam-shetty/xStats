use crate::utils;
pub use tree_sitter::{Language, Parser, Tree};

pub fn get_grammar_pairs() -> Vec<(&'static str, Language)> {
    vec![
        ("Java", tree_sitter_java::LANGUAGE.into()),
        ("Python", tree_sitter_python::LANGUAGE.into()),
    ]
}

pub struct TSParser {
    name: &'static str,
    parser: Parser,
}

impl TSParser {
    pub fn new(name: &'static str, grammar: Language) -> Self {
        let mut parser = Parser::new();
        parser
            .set_language(&grammar)
            .expect("Error setting language");
        Self { name, parser }
    }
}

pub struct TSParsers {
    parsers: Vec<TSParser>,
}

impl TSParsers {
    pub fn new() -> Self {
        let parsers = get_grammar_pairs()
            .into_iter()
            .map(|(name, grammar)| TSParser::new(name, grammar))
            .collect();
        Self { parsers }
    }

    fn get_parser(&mut self, name: &str) -> Option<&mut Parser> {
        self.parsers
            .iter_mut()
            .find(|p| p.name == name)
            .map(|p| &mut p.parser)
    }

    pub fn generate_tree(&mut self, name: &str, file_path: &str) -> Option<Tree> {
        let parser = self.get_parser(name)?;
        let source_code = utils::read_file(file_path);
        parser.parse(source_code, None)
    }
}
