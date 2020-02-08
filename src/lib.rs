#![allow(unused)]

/// Methods are covariant with respect to their return types:
///
/// ```text
///     Y <: X
/// --------------
/// A → Y <: A → X
/// ```
///
/// i.e. A method returning a `Y` can be used as if it is returning
/// an `X`.
///
/// Note that you can think of lifetimes as traits: for example,
/// the way you declare a lifetime outlives another (`'cat: 'animal`)
/// is also how you declare a trait is a subtype of another
/// (`Cat: Animal`).
pub fn covariance<'animal, 'cat: 'animal>() {
    let mut return_type: fn() -> &'animal ();
    //      Cat <: Animal
    // -----------------------
    // () → Cat <: () → Animal
    return_type = (|| &()) as fn() -> &'animal ();
    return_type = (|| &()) as fn() -> &'cat ();
    // A method returning a `Cat` can be used as if it is returning
    // an `Animal`.
}

/// Methods are contravariant with respect to their arguments types:
///
/// ```text
///     B <: A
/// --------------
/// A → X <: B → X
/// ```
///
/// i.e. A method accepting an `A` as an argument can be used as if
/// it is accepting a `B`.
///
/// Note that you can think of lifetimes as traits: for example,
/// the way you declare a lifetime outlives another (`'cat: 'animal`)
/// is also how you declare a trait is a subtype of another
/// (`Cat: Animal`).
pub fn contravariance<'animal, 'cat: 'animal>() {
    let mut argument_type: fn(&'cat ());
    //         Cat <: Animal
    // -----------------------
    // Animal → () <: Cat → ()
    argument_type = (|_| ()) as fn(&'cat ());
    argument_type = (|_| ()) as fn(&'animal ());
    // A method accepting an `Animal` as an argument can be
    // used as if it is accepting a `Cat`.
}
