/*
    Memory Refresh
    --------------

    Memory is stored using binary
    - Bits 0 or 1

    Computer optimized for bytes
    - 1 byte == 8 contiguous bits

    Fully Contiguous

    Addresses
    ---------
    - all data in memory has an "address"
    - used to locate data
    - always the same - only data changes
    - usually dont utilize addresses directly
      variables handle most of the work

    Offsets
    -------
    - items can be located at an adress using an "offset"
    - offets begin at 0
    - represent the number of bytes away from the original address
      normally deal with indexes instead

*/
fn main() {
    println!("Hello, world!");
}
