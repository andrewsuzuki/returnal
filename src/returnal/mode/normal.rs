use keyboard::Key;
use super::{Mode};

#[derive(Copy, Clone, RustcEncodable)]
pub struct NormalMode;

impl NormalMode {
    pub fn new() -> Self {
        NormalMode
    }
}

impl Mode for NormalMode {
    fn name(&self) -> &'static str {
        "normal"
    }

    fn receive_key(&self, key: Key) {

    }
}
