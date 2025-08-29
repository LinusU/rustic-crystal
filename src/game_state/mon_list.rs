use std::marker::PhantomData;

pub trait MonListItem<'a> {}

pub struct MonList<'a, T: MonListItem<'a>, const N: usize> {
    data: &'a [u8],
    _marker: PhantomData<T>,
}

impl<'a, T: MonListItem<'a>, const N: usize> MonList<'a, T, N> {
    pub fn new(data: &'a [u8]) -> Self {
        Self {
            data,
            _marker: PhantomData,
        }
    }

    pub fn len(&self) -> usize {
        self.data[0] as usize
    }
}
