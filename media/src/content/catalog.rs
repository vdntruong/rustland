// super is used to access parent module
use super::media::Media;

#[derive(Debug)]
pub enum MightHaveAValue<'a> {
    ThereIsAValue(&'a Media),
    NoValueAvailable,
}

#[derive(Debug)]
pub struct Catalog {
    items: Vec<Media>,
}

impl Catalog {
    pub fn new() -> Self {
        Catalog { items: vec![] }
    }

    pub fn add(&mut self, media: Media) {
        self.items.push(media);
    }

    pub fn get_by_index(&self, index: usize) -> MightHaveAValue {
        if self.items.len() > index {
            return MightHaveAValue::ThereIsAValue(&self.items[index]);
        }
        MightHaveAValue::NoValueAvailable
    }
}
