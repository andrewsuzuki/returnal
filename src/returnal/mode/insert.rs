use keyboard::Key;
use super::{Mode};

#[derive(Copy, Clone, RustcEncodable)]
pub struct InsertMode;

impl InsertMode {
    pub fn new() -> Self {
        InsertMode
    }
}

impl Mode for InsertMode {
    fn name(&self) -> &'static str {
        "insert"
    }

    fn receive_key(&self, key: Key) {

    }
}
