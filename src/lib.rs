#![allow(unused)]

/// Methods are covariant with respect to their return types:
///
/// ```text
///     Y <: X
/// --------------
/// A → Y <: A → X
/// ```
///
/// i.e. A method returning a `Y` can be declared as returning an `X`.
pub fn covariance() {
    let mut return_type: fn() -> &'static dyn Animal;
    //      Cat <: Animal
    // -----------------------
    // () → Cat <: () → Animal
    return_type = || -> &'static dyn Animal { &SOME_ANIMAL };
    return_type = || -> &'static dyn Animal { &SOME_CAT };
    // A method returning a `Cat` can be declared as returning
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
/// i.e. A method accepting an `A` as an argument can be declared as
/// accepting a `B`.
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
    // declared as returning a `Cat`.
}

trait Animal {}
trait Cat: Animal {}

struct SomeAnimal;
struct SomeCat;

static SOME_ANIMAL: SomeAnimal = SomeAnimal;
static SOME_CAT: SomeCat = SomeCat;

impl Animal for SomeAnimal {}
impl Animal for SomeCat {}
impl Cat for SomeCat {}
