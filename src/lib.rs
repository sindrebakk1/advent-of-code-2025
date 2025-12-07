pub mod template;

pub struct Stack<T>(Vec<T>);

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack(Vec::new())
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Stack(Vec::with_capacity(capacity))
    }

    pub fn from_vec(vec: Vec<T>) -> Stack<T> {
        Stack(vec)
    }

    pub fn push(&mut self, item: T) {
        self.0.push(item);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.0.pop()
    }

    pub fn peek(&self) -> Option<&T> {
        self.0.last()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn size(&self) -> usize {
        self.0.len()
    }

    pub fn as_slice(&self) -> &[T] {
        self.0.as_slice()
    }
}

impl<T> Default for Stack<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> FromIterator<T> for Stack<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self(iter.into_iter().collect())
    }
}
