pub fn let_let_let_me_in() {
    println!("Lets make a function that takes a closure");

    let add_one_closure = |x: i32| x + 1;
    let result: i32 = call_with_one(add_one_closure);
    println!("result of method taking a closure: {}", result);


    // so what is the advantage of a closure,
    // dynamic execution
    //
    // so how about a way of formatted different names for a struct

    // so we need a Person struct
    //
    // and want to call print name and pass a template
    //
    // and the template is a closure to format printing something

    let david = Person { name: "David".to_string() };

    let fancy_name = |person: Person| {
        let mut cloned_name: String = person.name.clone();
        cloned_name.push_str(" Esquire");
        cloned_name
    };

    // let result = fancy_name(david);
    // println!("Result {:?}", result);

    print_name(fancy_name(david));
}

struct Person {
    name: String
}


// This is confusing why this doesn't work
// the trait `for<'r> core::ops::FnOnce<(&'r str,)>` is not
// implemented for the type `collections::string::String`')'`
fn print_name<F>(name_format: F)
    where F : Fn(&str) -> String {

    println!("I am print name");
}

fn call_with_one<F>(some_closure: F) -> i32
    where F : Fn(i32) -> i32 {

    some_closure(1)
}
