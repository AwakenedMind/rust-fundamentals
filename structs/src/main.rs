// structs
// 1. all or nothing can not have some pieces of data and not others
// 2. each piece of data is called a "field"
// 3. similar data can be grouped together

struct GroceryItem {
    stock: i32,
    price: f64,
}

fn main() {
    let cereal = GroceryItem {
        stock: 10,
        price: 8.99,
    };

    println!("stock: {:?}", cereal.price)
}
