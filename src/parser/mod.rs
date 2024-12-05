use crate::utils;
pub use tree_sitter::{
    Language, Node, Parser, Query, QueryCaptures, QueryCursor, QueryMatches, Tree,
};

pub fn get_grammar_info() -> Vec<(&'static str, Language, Vec<&'static str>)> {
    vec![
        ("Java", tree_sitter_java::LANGUAGE.into(), vec![".java"]),
        ("Python", tree_sitter_python::LANGUAGE.into(), vec![".py"]),
    ]
}

pub struct TSParser {
    name: &'static str,
    language: Language,
    parser: Parser,
    supported_extensions: Vec<&'static str>,
}

impl TSParser {
    pub fn new(name: &'static str, grammar: Language) -> Self {
        let language = grammar;

        let mut parser = Parser::new();
        parser
            .set_language(&language)
            .expect("Error setting language");

        Self {
            name,
            language,
            parser,
            supported_extensions: vec![],
        }
    }

    /// Query the syntax tree for matches
    pub fn query_tree<'a>(
        &self,
        node: &Node,
        tree: &'a Tree,
        source_code: &'a str,
        query_string: &str,
    ) -> Vec<(Node<'a>, String)> {
        let query = Query::new(&self.language, query_string).expect("Invalid query");
        let mut query_cursor = QueryCursor::new();

        let mut results = Vec::new();
        for (query_match, index) in query_cursor.captures(&query, *node, source_code.as_bytes()) {
            let captures = query_match.captures;
            for capture in captures {
                let tag = query.capture_names()[capture.index as usize].to_string();
                let node = tree
                    .root_node()
                    .descendant_for_byte_range(capture.node.start_byte(), capture.node.end_byte())
                    .unwrap();
                results.push((node, tag));
            }
        }
        results
    }
}

pub struct TSParsers {
    ts_parsers: Vec<TSParser>,
}

impl TSParsers {
    pub fn new() -> Self {
        let ts_parsers = get_grammar_info()
            .into_iter()
            .map(|(name, grammar, extensions)| {
                let mut parser = TSParser::new(name, grammar);
                parser.supported_extensions = extensions;
                parser
            })
            .collect();
        Self { ts_parsers }
    }

    pub fn get_parser(&self, language: &str) -> Option<&TSParser> {
        self.ts_parsers
            .iter()
            .find(|parser| parser.name == language)
    }

    pub fn generate_tree(&mut self, file_path: &str) -> Option<(&'static str, Tree, String)> {
        let file_extension = utils::get_file_extension(file_path);

        for ts_parser in &mut self.ts_parsers {
            if ts_parser
                .supported_extensions
                .contains(&file_extension.as_str())
            {
                let source_code = utils::read_file(file_path);
                if let Some(tree) = ts_parser.parser.parse(&source_code, None) {
                    return Some((ts_parser.name, tree, source_code));
                }
            }
        }
        None
    }
}
