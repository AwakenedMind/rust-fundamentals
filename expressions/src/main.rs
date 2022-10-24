fn main() {
    let my_num = 3;

    let is_lt_5 = if my_num < 5 { true } else { false };

    println!("{}", is_lt_5);

    // expressions
    let message = match my_num {
        1 => "hello",
        2 => "goodbye",
        _ => "idk",
    };

    println!("{}", message);
}
