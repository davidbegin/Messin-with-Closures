pub fn let_let_let_me_in() {
    println!("Lets make a function that takes a closure");

    let add_one_closure = |x: i32| x + 1;
    let result: i32 = call_with_one(add_one_closure);
    println!("result of method taking a closure: {}", result);

    let mut name = "Juicy J";
    let mut fancy_strang_closure = |x: String| {
        let result = x.clone().push_str(name);
        println!("result: {:?}", x);
        1
    };

    modifies_a_strang(fancy_strang_closure);
}

fn call_with_one<F>(some_closure: F) -> i32
    where F : Fn(i32) -> i32 {

    some_closure(1)
}


// so this method takes a closure
//
// and that closure is one that takes a &str
// which in this case is "Esquire"
fn modifies_a_strang<F>(strang_closure: F) -> i32
    where F : Fn(String) -> i32 {

    let strang = "Mr. ".to_string();

    strang_closure(strang)
}
