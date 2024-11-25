use crate::parser::Tree;

pub struct CodeMetrics {
    pub file_path: String,
    pub loc: u32,
}

impl CodeMetrics {
    pub fn new(file_path: String) -> CodeMetrics {
        CodeMetrics { file_path, loc: 0 }
    }

    pub fn add_line(&mut self) {
        self.loc += 1;
    }

    pub fn set_loc(&mut self, loc: u32) {
        self.loc = loc;
    }

    pub fn calculate_loc(&mut self, tree: &Tree) {
        let root_node = tree.root_node();
        let start_line = root_node.start_position().row;
        let end_line = root_node.end_position().row;
        self.loc = (end_line - start_line + 1) as u32;
    }
}
