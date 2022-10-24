enum Access {
    Admin,
    Manager,
    Employee,
    User,
}

// tuples ()
fn one_two_three() -> (i32, i32, i32) {
    (1, 2, 3)
}

fn main() {
    let numbers = one_two_three();

    // destructure return values
    let (x, y, z) = one_two_three();

    println!("{:?}, {:?}", x, numbers.0);

    let (employee, access) = ("Jake", Access::Admin);
}
