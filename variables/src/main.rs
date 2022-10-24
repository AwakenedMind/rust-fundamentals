fn main() {

    // by default variables are immutable so we need to specify if a variable is mutable
    let mut name = "Rob";
    // std::fmt module https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/std/fmt/index.html
    println!("{}", name);

    // "mutate" the value 
    name = "James";
    println!("{}", name);

    // nick_name points to the same memory address as name
    let nick_name = name ;

    println!("{}", nick_name);

}
