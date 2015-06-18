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

    let plus_two = |x| {
        let mut result: i32 = x;
        result += 2;
        result
    };

    type_printer::print_type_of(&plus_two);
    // #=> [closure((i32,)) -> i32]

    // So you don't have to declare returns for closures
    // but they exist for the type?!?

    // type annotations or generic parameter binding required
    // let plus_three = |x| println!("hell yeah");

    // alright time for the main event

    println!("\n================================================");
    the_main_event();
}

fn the_main_event() {
    println!("
             Closures close over thier environment.
             And I do not know how that is going to interact with
             borrow and move semantics.
    ");

    let mut num = 5;

    // so plus_num is borrowing of num here
    {
        let plus_num = |x: i32| x + num;
    }

    let y = &mut num;

    // but it can also take ownership if need me
}
