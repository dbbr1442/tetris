#[derive(Clone, Copy, Debug)]
pub struct Block {
    pub location: (i32, i32),
}

impl Block {
    pub fn new(location: (i32, i32)) -> Self {
        let empty_block = Block {location};
        empty_block
    }
}
