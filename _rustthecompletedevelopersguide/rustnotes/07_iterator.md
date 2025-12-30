# Iterator

Iterator is a trait that can be used to iterate over a collection of items.

To get an iterator, we use the 'iter' methods:

- iter() -> Immutable iterator, this give your the reference of each element
- iter_mut() -> Mutable iterator, this give your the mutable reference of each element
- into_iter() -> Consuming iterator, this give your the ownership of each element

In Rust, Iterator have common methods:

- next()
- count()
- collect()
- sum()

With elements:

- Pointer to Data
- Pointer to Current Position
- Pointer to End

We usually don't call 'next' method directly. 

Option 1: Use a for loop

Option 2: Use iterator adapters and consumers like 'for each', 'collect', 'map', ...

- map is an iterator adaptor.

Adaptors create a step in a processing pipeline, but don't actually cause any iteration to happen until a consumer is called.

- for_each is an iterator consumer.

Consumers trigger the iteration and cause the adaptor to be applied to each item.

Consumers are methods that consume the iterator and produce a result.

## Collect

Collect is an iterator consumer that takes an iterator and returns a collection.

The result of collect can be a vector, a hash map, a set, a tuple, a struct, etc...

What determine return type of collect?

- The return type of the function
- The variable type annotation (`let vtr: Vec<String> = ...collect();`)
- The 'Turbofish' syntax (`.collect::<Type>()`)

## find

`find` is an **iterator consumer** that takes an iterator and returns the first element that satisfies a condition.

The `find` method returns an `Option` type, which is an enum that can be either `Some` or `None`.

## into_iter

`Into_iter()` will give you something different depending on how its called.

`&sth.into_iter()` -> Iterator created out of a reference -> Iterator will produce refs to each value.

`&mut sth.into_iter()` -> Iterator created out of a mutable reference -> Iterator will produce mutable refs to each value.

`sth.into_iter()` -> Iterator created out of a value -> Iterator will take **ownership** of each value.

## map_or

`map_or` is a method that takes a closure and a default value.
