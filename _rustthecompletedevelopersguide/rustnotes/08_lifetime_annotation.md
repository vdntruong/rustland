# Lifetime Annotation (Lifetime)

## What is Lifetime Annotation?

Lifetime Annotation is a way to tell the compiler about the relationship between references.

Help the compiler make sure refs won't outlive the value they refer to.

Hardest part: This will seem like something the compiler should do on its own.

If you have a function that takes in two or more refs and returns a ref, Rust will make a huge assumption.

> Rust assumes that the return ref will point at data referred to by one of the arguments.

Rust will not analyze the body of the function to figure out whether the return ref is pointing at the first or second argument.

To clarify which ref the return ref is pointing at, we need to use lifetime annotation.

```rust
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
```

`'a` is the convention for lifetime annotations. We can name it anything we want, like `'b`, `'c`, etc...

Questions:
- Why does it matter whether the return ref points at the first or second argument?
- Why doesn't Rust analyze the function body to figure out whether the return ref points at the first or second argument?

## Lifetime Elision (Lifetime Ellipsis)

We have to think about annotations anytime your function receives a ref and returns a ref.

You can omit lifetime annotations in the following cases:

- Function that takes in one ref + any number of non-ref arguments and returns a ref
- Method that takes &Self and any number of other refs + returns a ref. (Rust assumes the return ref points at data referred to by &Self)

Omitting lifetime annotations is called "lifetime elision".