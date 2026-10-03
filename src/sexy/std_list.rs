//! `std::list<T>` with the iterator semantics the original relies on: iterators are node
//! handles that stay valid while other nodes are inserted or erased, and `end()` is a
//! sentinel. `WidgetContainer::UpdateAll` keeps such an iterator across child updates
//! that may add or remove widgets, so a plain `Vec` would change behavior.

/// A list iterator: a node handle. [`StdList::end`] is the sentinel.
pub type ListIter = u32;

#[derive(Clone, Debug)]
struct Node<T> {
    prev: ListIter,
    next: ListIter,
    value: Option<T>,
}

#[derive(Clone, Debug)]
pub struct StdList<T> {
    nodes: Vec<Node<T>>,
    free: Vec<ListIter>,
    len: usize,
}

impl<T> Default for StdList<T> {
    fn default() -> Self {
        StdList { nodes: vec![Node { prev: 0, next: 0, value: None }], free: Vec::new(), len: 0 }
    }
}

impl<T: Copy + PartialEq> StdList<T> {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
    pub fn begin(&self) -> ListIter {
        self.nodes[0].next
    }
    pub fn end(&self) -> ListIter {
        0
    }
    pub fn next(&self, it: ListIter) -> ListIter {
        self.nodes[it as usize].next
    }
    pub fn prev(&self, it: ListIter) -> ListIter {
        self.nodes[it as usize].prev
    }
    /// `*it`. Panics on `end()`, as dereferencing it is undefined in the original.
    pub fn get(&self, it: ListIter) -> T {
        self.nodes[it as usize].value.expect("dereferenced std::list end()")
    }
    pub fn set(&mut self, it: ListIter, v: T) {
        self.nodes[it as usize].value = Some(v);
    }
    pub fn front(&self) -> T {
        self.get(self.begin())
    }
    pub fn back(&self) -> T {
        self.get(self.prev(0))
    }
    /// `insert(where, value)`: inserts before `at`, returns the new node.
    pub fn insert(&mut self, at: ListIter, v: T) -> ListIter {
        let prev = self.nodes[at as usize].prev;
        let node = Node { prev, next: at, value: Some(v) };
        let id = if let Some(id) = self.free.pop() {
            self.nodes[id as usize] = node;
            id
        } else {
            self.nodes.push(node);
            (self.nodes.len() - 1) as ListIter
        };
        self.nodes[prev as usize].next = id;
        self.nodes[at as usize].prev = id;
        self.len += 1;
        id
    }
    pub fn push_back(&mut self, v: T) -> ListIter {
        self.insert(0, v)
    }
    pub fn push_front(&mut self, v: T) -> ListIter {
        let b = self.begin();
        self.insert(b, v)
    }
    /// `erase(it)`: returns the iterator after `it`.
    pub fn erase(&mut self, it: ListIter) -> ListIter {
        assert!(it != 0, "erase(end())");
        let Node { prev, next, .. } = self.nodes[it as usize];
        self.nodes[prev as usize].next = next;
        self.nodes[next as usize].prev = prev;
        self.nodes[it as usize].value = None;
        self.free.push(it);
        self.len -= 1;
        next
    }
    pub fn pop_front(&mut self) {
        let b = self.begin();
        self.erase(b);
    }
    pub fn clear(&mut self) {
        *self = Self::default();
    }
    /// `std::find(begin, end, v)`.
    pub fn find(&self, v: T) -> ListIter {
        let mut it = self.begin();
        while it != 0 && self.get(it) != v {
            it = self.next(it);
        }
        it
    }
    /// `remove(v)`: erases every element equal to `v`.
    pub fn remove(&mut self, v: T) {
        let mut it = self.begin();
        while it != 0 {
            if self.get(it) == v {
                it = self.erase(it);
            } else {
                it = self.next(it);
            }
        }
    }
    pub fn iter(&self) -> impl Iterator<Item = T> + '_ {
        let mut it = self.begin();
        std::iter::from_fn(move || {
            if it == 0 {
                None
            } else {
                let v = self.get(it);
                it = self.next(it);
                Some(v)
            }
        })
    }
}
