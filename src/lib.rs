use std::ops::Sub;

pub mod template;

pub mod dlx;

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

#[derive(Debug)]
pub struct DSU {
    parent: Vec<usize>,
    size: Vec<u64>,
}

impl DSU {
    pub fn new(n: usize) -> Self {
        let parent = (0..n).collect();
        let size = vec![1; n];
        Self { parent, size }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            let p = self.parent[x];
            self.parent[x] = self.find(p);
        }
        self.parent[x]
    }

    pub fn union(&mut self, a: usize, b: usize) {
        let mut ra = self.find(a);
        let mut rb = self.find(b);
        if ra == rb {
            return;
        }

        if self.size[ra] < self.size[rb] {
            std::mem::swap(&mut ra, &mut rb);
        }
        self.parent[rb] = ra;
        self.size[ra] += self.size[rb];
    }

    pub fn component_sizes(&mut self) -> Vec<u64> {
        let n = self.parent.len();
        let mut root_sizes = vec![0u64; n];
        for i in 0..n {
            let r = self.find(i);
            root_sizes[r] += 1;
        }
        root_sizes.into_iter().filter(|&s| s > 0).collect()
    }

    pub fn component_size(&mut self, x: usize) -> u64 {
        let r = self.find(x);
        self.size[r]
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct OrdF64(pub f64);

impl Eq for OrdF64 {}

impl PartialOrd for OrdF64 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for OrdF64 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.total_cmp(&other.0)
    }
}
