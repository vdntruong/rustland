mod content;

//  module : file : public object
use content::catalog::{Catalog, MightHaveAValue};
use content::media::Media;

fn print_media(audiobook: &Media) {
    println!("{:#?}", audiobook);
}

fn main() {
    let audiobook = Media::AudioBook {
        title: String::from("The Rust Programming Language"),
    };
    let goodmovie = Media::Movie {
        title: String::from("Goodbye, Lenin!"),
        director: String::from("Lars von Trier"),
    };
    let badbook = Media::Book {
        title: String::from("The Rust Programming Language"),
        author: String::from("Steve Klabnik and Carol Nichols"),
    };
    // unlabeled fields in an enum variant
    let podcast = Media::Podcast(1);
    let placeholder = Media::Placeholder;

    print_media(&audiobook);
    print_media(&goodmovie);
    print_media(&badbook);

    audiobook.description();
    goodmovie.description();
    badbook.description();

    let mut catalog = Catalog::new();
    catalog.add(audiobook);
    catalog.add(goodmovie);
    catalog.add(badbook);
    catalog.add(podcast);
    catalog.add(placeholder);

    // println!("{:#?}", catalog);
    println!("{:#?}", catalog.items.get(0));
    println!("{:#?}", catalog.items.get(100));

    // we have built-in Option type
    match catalog.items.get(100) {
        Some(media) => println!("{:#?}", media),
        None => println!("No media found"),
    }

    // get movie by index
    let item = catalog.get_by_index(1);
    println!("{:#?}", item);

    // match enum pattern
    match catalog.get_by_index(10) {
        MightHaveAValue::ThereIsAValue(media) => {
            println!("{:#?}", media)
        }
        MightHaveAValue::NoValueAvailable => {
            println!("No media found")
        }
    }

    // compare types
    let index = 4;
    if let MightHaveAValue::ThereIsAValue(media) = catalog.get_by_index(index) {
        println!("{:#?}", media)
    } else {
        println!("No media found for index {}", index)
    }

    let item = catalog.items.get(1);
    println!("{:#?}", item.unwrap());
}
