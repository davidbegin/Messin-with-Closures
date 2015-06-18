fn main() {
    println!("\nMessin' with Closures");
    println!("=====================\n");

    let plus_ten = |x: i32| x + 10;
    println!("100 plus 10 = {}", plus_ten(100));
}
