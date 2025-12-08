use std::ops::Sub;

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

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub struct IVec3 {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl IVec3 {
    pub fn new(x: i64, y: i64, z: i64) -> Self {
        IVec3 { x, y, z }
    }

    pub fn magnitude(&self) -> f64 {
        ((self.x.pow(2) + self.y.pow(2) + self.z.pow(2)) as f64).sqrt()
    }

    pub fn distance(self, other: IVec3) -> f64 {
        (self - other).magnitude()
    }
}

impl Sub<IVec3> for IVec3 {
    type Output = IVec3;

    fn sub(self, rhs: IVec3) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}
