fn main() {
    println!("Hello, world!");

    let numba = basic_for_loop(10);
    let numba2 = basic_while_loop(10);

    println!("forloop return val: {}", numba);
    println!("whileloop return val: {}", numba2);
}

fn basic_for_loop(numba: i32) -> i32 {
    let mut final_val = 0;

    // mutable iterator variable
    let mut a = 0;

    loop {
        if a == 5 {
            break;
        }

        println!("{:?}", a);

        a = a + 1;
        final_val = final_val + numba;
    }

    return final_val
}

fn basic_while_loop(numba: i32) -> i32 {
    let mut final_val = 0;

    // mutable iterator variable
    let mut a = 0;

    while a != 5 {
        println!("{:?}", a);

        a = a + 1;
        final_val = final_val + numba;
    }

    return final_val
}