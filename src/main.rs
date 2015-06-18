extern crate type_printer;

fn main() {
    println!("\nMessin' with Closures");
    println!("=====================\n");

    // so this is a very self explantory example
    //
    // it makes closures seem easy!
    //
    // but thats because I am not trying different things
    let plus_ten = |x: i32| x + 10;
    println!("100 plus 10 = {}", plus_ten(100));

    // What are some other examples of simple useless closures
    //
    // especially using different types,
    // multiple arguments
    // Generics?
    //
    // a challenge!

    // Challenge 1: Concat 2 strings!
    //
    //
    // Will not compile
    // let name_formalizer = |x: str| "Mr. ".push_str(x);
    // `str` does not have a constant size known at compile-time
    // the trait `core::marker::Sized` is not implemented for the type `str`


    // Will not compile
    // let name_formalizer = |x: &'static str| "Mr. ".push_str(x);
    // no method named `push_str` found for type `&'static str` in the current scope'`

    // This compiles!
    let name_formalizer = |x: &'static str| "Mr. ".to_owned().push_str(x);
    type_printer::print_type_of(&name_formalizer);
    // #=> [closure((&'static str,))]'))]
    // interesting!

    // So let me try and expound on what is happening

    // we are binding a variable to a closure, or something that has
    // the clojure type
    //
    // it takes a statically allocated str and adds it to a
    // Heap Allocated string

    // Challenge 2: Multiple Arguments

    let point_zone = |x: i8, y: i8| [x, y];
    type_printer::print_type_of(&point_zone);
    // #=> closure((i8, i8)) -> [i8; 2]]
    //
    // woaaaah!

    // Challenge 3: Generics! (will we need traits!?!?)
    // where should I declare my Generic
    //
    // I am truly stumped
    // let animal_whisperer = |animal: T| animal

    // ...it worked! Time to read the next line of the docs
}

