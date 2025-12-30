fn next_language<'a>(languages: &'a [String], current: &str) -> &'a str {
    let mut found = false;

    for lang in languages {
        if found {
            return lang;
        }

        if lang == current {
            found = true;
        }
    }

    // we assume the vector slice languages always has at least one element
    languages.last().unwrap()
}

// Rust assumes that the returned ref is tied to the input ref
fn last_language(languages: &[String]) -> &str {
    languages.last().unwrap()
}

fn longest_language<'a>(lang_a: &'a str, lang_b: &'a str) -> &'a str {
    if lang_a.len() >= lang_b.len() {
        lang_a
    } else {
        lang_b
    }
}

fn main() {
    let languages = vec![
        String::from("python"),
        String::from("go"),
        String::from("rust"),
    ];

    let result = next_language(&languages, "go");
    println!("{}", result);

    let longest = longest_language("python", "go");
    println!("{}", longest);
}
