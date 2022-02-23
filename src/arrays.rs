// Arrays - Fixed list where elements are the same data types

use std::mem::size_of_val as sizeof;

pub fn run() {
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

    // Re-assign value
    numbers[2] = 20;

    println!("{:?}", numbers);

    // Get single val
    println!("Single Value: {}", numbers[0]);

    // Get array length
    println!("Array Length: {}", numbers.len());

    // Arrays are stack allocated
    println!("Array occupies {} bytes", sizeof(&numbers));

    // Get Slice
    let slice: &[i32] = &numbers[1..4];
    println!("Slice: {:?}", slice);
}
