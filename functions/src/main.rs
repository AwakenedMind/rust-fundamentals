fn main() {
    println!("Hello, world!");

    let x = add(1, 2);
    let y = add(x, 3);
    let z = add(x, y);

    println!("{}", z);

}

// anatomy of a function

// 1) fn keyword
// 2) fn name 
// 3) parameters    param: data type 
// 4) return type using ->    -> return type
fn add(a: i32, b:i32) -> i32 {
    a + b
}