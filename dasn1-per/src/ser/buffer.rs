use bit_vec::BitVec;

#[derive(Debug, Default, Clone)]
pub struct Buffer(BitVec);

impl Buffer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_elem(n: usize, default: bool) -> Self {
        Self(BitVec::from_elem(n, default))
    }

    pub fn push_field_list(&mut self, target: Self) {
        for bit in target.into_inner() {
            self.0.push(bit);
        }
    }

    fn into_inner(self) -> BitVec {
        self.0
    }
}

impl std::ops::Deref for Buffer {
    type Target = BitVec;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Buffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
