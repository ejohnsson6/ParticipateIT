use std::fs;

pub struct fdfbuilder {
    content: String,
}

impl fdfbuilder {
    pub fn new() -> fdfbuilder {
        fdfbuilder {
            content: fs::read_to_string()
        }
    }
}