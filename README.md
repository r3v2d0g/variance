# Variance

## Notation

```text
A <: B           # trait A: B {}

A → B            # fn(A) -> B;

A true  B true   # if a && b
--------------   # {
(A and B) true   #     assert!(a & b);
                 # }
```

## Covariance

Methods are covariant with respect to their return types:

```text
    Y <: X
--------------
A → Y <: A → X
```

i.e. A method returning a `Y` can be used as if it is returning
an `X`.

###### NOTE: Rust applies covariance for the lifetimes of the methods' return types.

### Example: see [`lib.rs`](src/lib.rs#L18)

Note that you can think of lifetimes as traits: for example,
the way you declare a lifetime outlives another (`'cat: 'animal`)
is also how you declare a trait is a subtype of another
(`Cat: Animal`).

```rust
pub fn covariance<'animal, 'cat: 'animal>() {
    let mut return_type: fn() -> &'animal ();
    //      Cat <: Animal
    // -----------------------
    // () → Cat <: () → Animal
    return_type = (|| &()) as fn() -> &'animal ();
    return_type = (|| &()) as fn() -> &'cat ();
    // A method returning a `Cat` can be used as one returning
    // an `Animal`.
}
```

## Contravariance

Methods are contravariant with respect to their arguments types:

```text
    B <: A
--------------
A → X <: B → Y
```
i.e. A method accepting an `A` as an argument can be used as if
it is accepting a `B`.

###### NOTE: Rust applies contravariance for the lifetimes of the methods' arguments.

### Example: see [`lib.rs`](src/lib.rs#L44)

Note that you can think of lifetimes as traits: for example,
the way you declare a lifetime outlives another (`'cat: 'animal`)
is also how you declare a trait is a subtype of another
(`Cat: Animal`).

```rust
pub fn contravariance<'animal, 'cat: 'animal>() {
    let mut argument_type: fn(&'cat ());
    //         Cat <: Animal
    // -----------------------
    // Animal → () <: Cat → ()
    argument_type = (|_| ()) as fn(&'cat ());
    argument_type = (|_| ()) as fn(&'animal ());
    // A method accepting an `Animal` as an argument can be
    // used as one accepting a `Cat`.
}
```

## Resources

- [The Rustonomicon: Subtyping and Variance](https://doc.rust-lang.org/nightly/nomicon/subtyping.html)
- [RustFest: Felix Klock - Subtyping in Rust and Clarke's Third Law](https://www.youtube.com/watch?v=fI4RG_uq-WU)

## License

This project is licensed under the [MIT License](LICENSE.md).
