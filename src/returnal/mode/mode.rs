use keyboard::Key;

#[derive(Copy, Clone, Debug, RustcEncodable)]
pub enum ModeType {
    Normal,
    Insert
}

pub trait Mode {
    fn name(&self) -> &'static str;
    fn receive_key(&self, key: Key);
}
