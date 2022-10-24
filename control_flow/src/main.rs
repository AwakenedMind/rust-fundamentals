fn main() {
    println!("Hello, world!");

    basic_control_flow(499);
    basic_control_flow_2(38);
}

fn basic_control_flow(a: i32) {
    if a > 39 {
        println!("BIGG NUMBA")
    } else {
        println!("SMALL NUMBA")
    }
}

fn basic_control_flow_2(a: i32) {
    if a > 100 {
        println!("HUGGEEE NUMBA")
    } else if a > 50 {
        println!("BIGG NUMBA")
    } else {
        println!("SMALL NUMBAA")
    }
}
