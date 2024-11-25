pub struct CodeMetrics {
    pub loc: u32,
}

impl CodeMetrics {
    pub fn new() -> CodeMetrics {
        CodeMetrics { loc: 0 }
    }

    pub fn add_line(&mut self) {
        self.loc += 1;
    }

    pub fn set_loc(&mut self, loc: u32) {
        self.loc = loc;
    }
}
