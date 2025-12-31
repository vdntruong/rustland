use crate::container::Container;

pub struct Basket<T> {
    item: Option<T>, // Option enum is a Some or None
}

// Why we need two T here?
// the first T is the list of generic type parameters
// the second T is the type of the item in the basket
//
// this similar to the following:
// struct Basket<T> {
//     item: Option<T>,
// }
// OR
// fn solve<T: ToPrimitive, U: ToPrimitive>(a: T, b: U) -> f64 {}
impl<T> Basket<T> {
    pub fn new(item: T) -> Self {
        return Basket { item: Some(item) };
    }
}

// declare that Basket implements Container trait
impl<T> Container<T> for Basket<T> {
    fn get(&mut self) -> Option<T> {
        self.item.take()
    }

    fn put(&mut self, item: T) {
        self.item = Some(item);
    }

    fn is_empty(&self) -> bool {
        self.item.is_none()
    }
}
