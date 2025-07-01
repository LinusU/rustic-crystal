pub struct Box<'a> {
    data: &'a [u8],
}

impl<'a> Box<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data }
    }

    pub fn capacity(&self) -> u8 {
        20
    }

    pub fn len(&self) -> u8 {
        self.data[0]
    }

    pub fn is_full(&self) -> bool {
        self.len() >= self.capacity()
    }
}
