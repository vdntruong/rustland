// &Vec<String> allow a full reference to the vector
// &[String] can be called with neither a full reference or a slice (&Vec<String> or &Vec<String>[1..3])
fn print_elements(elements: &[String]) {
    // for element in elements {
    //     println!("{}", element);
    // }

    // for_each is an iterator consumer
    elements.iter().for_each(|el| println!("{}", el));

    // map is an iterator adapter
    elements
        .iter()
        .map(|el| format!("{} {}", el, el))
        .for_each(|el| println!("{}", el));
}

// we want to modify the vector -> use mutable reference
fn shorten_strings(elements: &mut [String]) {
    // we use iter_mut to get a mutable iterator
    elements.iter_mut().for_each(|el| {
        el.truncate(1);
    });
}

// &[String] is a vector slice, we use this to be able to call the function with a slice of the vector
fn to_uppercase(elements: &[String]) -> Vec<String> {
    elements.iter().map(|el| el.to_uppercase()).collect()
}

// move_elements is a function that moves all elements from one vector to another
fn move_elements(vec_a: Vec<String>, vec_b: &mut Vec<String>) {
    vec_a.into_iter().for_each(|el| vec_b.push(el));
}

// explode is a function that explodes a string into a vector of strings
// chars is an iterator adapter, its job is to split the string into characters
fn explode(elements: &[String]) -> Vec<Vec<String>> {
    elements
        .iter()
        .map(|el| el.chars().map(|c| c.to_string()).collect())
        .collect()
}

// find_color_or is a function that finds a color in a vector of strings
// search arg we use to find the color -> read-only reference
// fallback arg we use to return a default value if the color is not found -> read-only reference
fn find_color_or(elements: &[String], search: &str, fallback: &str) -> String {
    elements
        .iter()
        .find(|el| el.contains(search))
        .map_or(String::from(fallback), |el| el.to_string())
}

fn main() {
    let mut color = vec![
        String::from("red"),
        String::from("blue"),
        String::from("green"),
    ];
    print_elements(&color);

    // shorten_strings(&mut color[0..2]);

    // let upper = to_uppercase(&color);
    // print_elements(&upper);

    let mut dest = vec!["yellow".to_string()];
    move_elements(color, &mut dest);

    // print_elements(&color);
    print_elements(&dest);

    let exploded = explode(&dest);
    println!("{:#?}", exploded);
    // &color[1..3] is a slice of the vector, vector slice is a reference to a part of the vector
    // print_elements(&color[1..3]);

    let found_color = find_color_or(&dest, "re", "Orange");
    println!("{}", found_color);
    //     let mut color_iter = color.iter();
    //     println!("{:?}", color_iter.next());
    //     println!("{:?}", color_iter.next());
    //     println!("{:?}", color_iter.next());
    //     println!("{:?}", color_iter.next());
}
