
fn main() {
    println!("Hello, world!");

    // match will be checked by the compiler 
    // if a new possibilyt is added, you will be notified when this occurs!

    // else if is NOT checked by the compiler, if new possibility is added, your code may contain a bug

    let some_bool = true;

    match some_bool {
        true -> println!("TRUEEE")
        false -> println!("FALSEE")
    }

    let some_int = 3;

    match some_int {
        1 -> println!("ONEEEE")
        2 -> println!("TWOOOO")
        3 -> println!("THREEE")
        // use _ to "match" anything else
        _ -> println!("ANYTHING ELSE")
    }
}
