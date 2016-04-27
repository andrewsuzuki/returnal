use ws::{Sender, Message, Result};
use mode::{Mode, NormalMode};

pub struct Cursor {
    x: usize,
    y: usize
}

impl Cursor {
    pub fn new(x: usize, y: usize) -> Self {
        Cursor { x: x, y: y }
    }
}

pub struct Editor<'e> {
    sender: Sender,
    cursor: Cursor,
    mode: Box<Mode + 'e>,
    content: String // TODO Rope
    // view: View
}

impl<'e> Editor<'e> {
    pub fn new(sender: Sender) -> Self {
        Editor {
            sender: sender,
            cursor: Cursor::new(0, 0),
            mode: Box::new(NormalMode::new()),
            content: String::from("")
        }
    }

    pub fn receive(&self, msg: Message) -> Result<()> {
        self.sender.send(msg)
    }
}
