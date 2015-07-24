extern crate type_printer;

pub fn an_exploration() {
    println!("An exploration");

    let plus_one = |x: i32| x + 1;
    let fake_plus_one = |x: i32| plus_one(x);

    type_printer::print_type_of(&plus_one);
    // [closure((i32,)) -> i32]
    //
    // a closure that takes an i32 and returns an i32

    // the body is an expression

    // This did not create a new scope
    // that plus one could even be called in here
    let plus_two = |x: i32| {
        let mut result = x;
        let first_result = fake_plus_one(x);
        let second_result = fake_plus_one(x);
        second_result
    };

    plus_two(10);
}


