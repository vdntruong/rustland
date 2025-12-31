use super::container::Container;

pub struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new(items: Vec<T>) -> Self {
        Stack { items }
    }
}

// declare that Stack implements Container trait
// Stack<T> is the type that implements Container<T>
// then all methods can omit the `pub` keyword
impl<T> Container<T> for Stack<T> {
    fn get(&mut self) -> Option<T> {
        self.items.pop()
    }

    fn put(&mut self, item: T) {
        self.items.push(item);
    }

    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}
