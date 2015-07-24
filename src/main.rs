#![allow(unused_variables, unused_mut, dead_code)]
extern crate type_printer;
mod functions_that_take_closures;
mod pondering_closures;

fn main() {
    println!("\nMessin' with Closures");
    println!("=====================\n");

    pondering_closures::an_exploration();

    // so this is a very self explantory example
    //
    // it makes closures seem easy!
    //
    // but thats because I am not trying different things
    // let plus_ten = |x: i32| x + 10;
    // println!("100 plus 10 = {}", plus_ten(100));

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
    // let name_formalizer = |x: &'static str| "Mr. ".to_owned().push_str(x);
    // type_printer::print_type_of(&name_formalizer);
    // #=> [closure((&'static str,))]'))]
    // interesting!

    // So let me try and expound on what is happening

    // we are binding a variable to a closure, or something that has
    // the clojure type
    //
    // it takes a statically allocated str and adds it to a
    // Heap Allocated string

    // Challenge 2: Multiple Arguments

    // let point_zone = |x: i8, y: i8| [x, y];
    // type_printer::print_type_of(&point_zone);
    // #=> closure((i8, i8)) -> [i8; 2]]
    //
    // woaaaah!

    // Challenge 3: Generics! (will we need traits!?!?)
    // where should I declare my Generic
    //
    // I am truly stumped
    // let animal_whisperer = |animal: T| animal

    // ...it worked! Time to read the next line of the docs

    // let plus_two = |x| {
    //     let mut result: i32 = x;
    //     result += 2;
    //     result
    // };
    //
    // type_printer::print_type_of(&plus_two);
    // #=> [closure((i32,)) -> i32]

    // So you don't have to declare returns for closures
    // but they exist for the type?!?

    // type annotations or generic parameter binding required
    // let plus_three = |x| println!("hell yeah");

    // alright time for the main event

    // println!("\n================================================");
    // the_main_event();
    // young_programmer_move_that_ownership_of_environment();
    // functions_that_take_closures::let_let_let_me_in();
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

fn young_programmer_move_that_ownership_of_environment() {
    let num: i32 = 5;

    // So this compiles fine
    let owns_num = move |x: i32| x + num;
    // and all it is doing is taking ownership of that 5


    let mut num: i32 = 5;

    // so move let the num unchanged?
    // because we took ownership of a copy?
    {
        // let mut add_num = |x: i32| num += x;
        let mut add_num = move |x: i32| num += x;
        add_num(5);
    }

    // Words straight form the Docs

    // Another way to think about move closures:
    //   they give a closure its own stack frame.
    //
    // Without move, a closure may be tied to the stack frame that created it,
    // while a move closure is self-contained.
    // This means that you cannot generally return a non-move closure from a function

    println!("{}", num);

    // More words from the docs:
    //
    // Rust’s implementation of closures is a bit different than other languages.
    // They are effectively syntax sugar for traits.
    lemme_try_and_break_this_down();
}

// I need to come back to this when I understnd extern "rust-call" more
fn lemme_try_and_break_this_down() {
    // pub trait Fn<Args> : FnMut<Args> {
    //     extern "rust-call" fn call(&self, args: Args) -> Self::Output;
    // }
    //
    // pub trait FnMut<Args> : FnOnce<Args> {
    //     extern "rust-call" fn call_mut(&mut self, args: Args) -> Self::Output;
    // }
    //
    // pub trait FnOnce<Args> {
    //     type Output;
    //
    //     extern "rust-call" fn call_once(self, args: Args) -> Self::Output;
    // }


    // #1:

    // So we have a public trait called Fn
    // this trait also requires that is declared for a type

    // that type is a genric call Args
    //
    // this trait also requires that another trait is also implemented for
    // the type you are implement for.
    //
    // and that trait is FnMut and it takes the same type as the Generic the main
    // trait takes
    //
    // Ok lost me at extern "rust-call"
    //
    // can I look beyond?
    //
    // and what is Self::Output?
    //
    // alright rabbit hole time
    // pub trait Fn<Args> : FnMut<Args> {
    //     extern "rust-call" fn call(&self, args: Args) -> Self::Output;
    // }
}
