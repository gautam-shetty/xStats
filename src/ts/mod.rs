use crate::config;
use crate::utils;
use config::Language;
use std::collections::HashMap;
pub use tree_sitter::{
    InputEdit, Language as TSLanguage, Node, Parser, Point, Query, QueryCaptures, QueryCursor,
    QueryMatches, Tree,
};

pub fn get_grammar_info() -> Vec<(Language, TSLanguage, Vec<&'static str>)> {
    vec![
        (
            Language::Java,
            tree_sitter_java::LANGUAGE.into(),
            vec![".java"],
        ),
        (
            Language::Python,
            tree_sitter_python::LANGUAGE.into(),
            vec![".py"],
        ),
    ]
}

pub struct TSParser {
    language: TSLanguage,
    parser: Parser,
    supported_extensions: Vec<&'static str>,
}

impl TSParser {
    pub fn new(grammar: TSLanguage) -> Self {
        let language = grammar;

        let mut parser = Parser::new();
        parser
            .set_language(&language)
            .expect("Error setting language");

        Self {
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
    ts_parsers: HashMap<Language, TSParser>,
}

impl TSParsers {
    pub fn new() -> Self {
        let ts_parsers = get_grammar_info()
            .into_iter()
            .map(|(name, grammar, extensions)| {
                let mut parser = TSParser::new(grammar);
                parser.supported_extensions = extensions;
                (name, parser)
            })
            .collect::<HashMap<Language, TSParser>>();
        Self { ts_parsers }
    }

    pub fn get_parser(&self, language: &Language) -> Option<&TSParser> {
        self.ts_parsers.get(language)
    }

    pub fn generate_tree(
        &mut self,
        trees_bin: &mut TSTreesBin,
        file_path: &str,
        content: Option<String>,
    ) -> Option<(Language, Tree, String)> {
        let file_extension = utils::get_file_extension(file_path);

        for (lang, ts_parser) in &mut self.ts_parsers {
            if ts_parser
                .supported_extensions
                .contains(&file_extension.as_str())
            {
                let source_code = match content {
                    Some(ref content) => content.clone(),
                    None => utils::read_file(file_path),
                };

                if let Some(tree) = Self::parse_with_ts(&mut ts_parser.parser, &source_code, None) {
                    return Some((lang.clone(), tree, source_code.to_string()));
                }
            }
        }
        None
    }

    pub fn generate_tree_from_blob(
        &mut self,
        trees_bin: &mut TSTreesBin,
        file_path: &str,
        source_code: &str,
    ) -> Option<(Language, Tree, String)> {
        let file_extension = utils::get_file_extension(file_path);

        for (lang, ts_parser) in &mut self.ts_parsers {
            if ts_parser
                .supported_extensions
                .contains(&file_extension.as_str())
            {
                let source_code = source_code.to_string();
                let old_tree = match trees_bin.get_tree(file_path) {
                    Some(tree) => Some(tree),
                    None => None,
                };

                if let Some(tree) =
                    Self::parse_with_ts(&mut ts_parser.parser, &source_code, old_tree.as_deref())
                {
                    return Some((lang.clone(), tree, source_code));
                }
            }
        }
        None
    }

    fn parse_with_ts(
        parser: &mut tree_sitter::Parser,
        source_code: &str,
        old_tree: Option<&Tree>,
    ) -> Option<Tree> {
        parser.parse(source_code, old_tree)
    }

    pub fn get_all_supported_extensions(&self) -> Vec<&'static str> {
        self.ts_parsers
            .iter()
            .flat_map(|(_, parser)| parser.supported_extensions.iter())
            .cloned()
            .collect()
    }
}

/// A structure that holds the history of trees.
///
/// # Fields
///
/// * `trees` - A `HashMap` where the key is a `String` representing the path,
///   and the value is a `Tree` which is of the Tree-sitter tree type.
pub struct TSTreesBin {
    trees: HashMap<String, Tree>,
}

impl TSTreesBin {
    pub fn new() -> Self {
        Self {
            trees: HashMap::new(),
        }
    }

    pub fn get_trees(&self) -> &HashMap<String, Tree> {
        &self.trees
    }

    pub fn num_trees(&self) -> usize {
        self.trees.len()
    }

    pub fn get_tree(&mut self, file_path: &str) -> Option<&mut Tree> {
        self.trees.get_mut(file_path)
    }

    pub fn delete_tree(&mut self, file_path: &str) {
        self.trees.remove(file_path);
    }

    pub fn insert_tree(&mut self, file_path: &str, tree: Tree) {
        self.trees.insert(file_path.to_string(), tree);
    }

    pub fn get_stats(&self) {
        let trees = &self.get_trees();
        let num_trees = self.num_trees();

        let history_size = std::mem::size_of_val(&trees);
        let entries_size: usize = trees
            .iter()
            .map(|(k, v)| std::mem::size_of_val(k) + std::mem::size_of_val(v))
            .sum();
        let total_size = history_size + entries_size;
        println!("Number of trees in TSHistory: {}", num_trees);
        println!("Size of the HashMap TSHistory: {} bytes", total_size);
    }
}
