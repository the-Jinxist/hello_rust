
use std::mem;

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

pub fn execute_primitives() {

    let inv = 98.0;
    let logical: bool = false;

    let age: u32 = 20;
    let age_two: i64 = 20;
    let sexy_flow: f64 = 10.54;

    let mut inferred_type = 12; // Type i64 is inferred from another line.
    println!("inferred_type: {} ", inferred_type);

    inferred_type = 4294967296i64;

    // A mutable variable's value can be changed.
    let mut mutable = 12; // Mutable `i32`
    println!("mutable: {} ", mutable);
    mutable = 21;

    let new_type = inferred_type + mutable;

    let array_ski: [i16; 7] = [3, 4, 5, 6, 7, 8, 9];

    let my_tuple = (5u32, 1u8, true, -5.04f32);

    println!("inv: {} ", inv);
    println!("logical: {} ", logical);
    println!("age: {} ", age);
    println!("age_two: {} ", age_two);
    println!("sexy_flow: {} ", sexy_flow);
    println!("inferred_type: {} ", inferred_type);
    println!("new_type: {} ", new_type);
    println!("mutable: {} ", mutable);
    println!("array_ski: {:?} ", array_ski);
    println!("my_tuple: {:?} ", my_tuple);

}

pub fn execute_lieral_operators() {
    // Integer addition
    println!("1 + 2 = {}", 1u32 + 2);

    // Integer subtraction
    println!("1 - 2 = {}", 1i32 - 2);
    // TODO ^ Try changing `1i32` to `1u32` to see why the type is important

    // Scientific notation
    println!("1e4 is {}, -2.5e-3 is {}", 1e4, -2.5e-3);

    // Short-circuiting boolean logic
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // Use underscores to improve readability!
    println!("One million is written as {}", 1_000_000u32);
}

pub fn execute_tuple() {
    // A tuple with a bunch of different types.
    let long_tuple = (1u8, 2u16, 3u32, 4u64,
                      -1i8, -2i16, -3i32, -4i64,
                      0.1f32, 0.2f64,
                      'a', true);

    // Values can be extracted from the tuple using tuple indexing.
    println!("Long tuple first value: {}", long_tuple.0);
    println!("Long tuple second value: {}", long_tuple.1);

    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

    // Tuples are printable.
    println!("tuple of tuples: {:?}", tuple_of_tuples);
    let values = return_tuple();
    println!("tuple of tuples: {:?}",values );

    // To create one element tuples, the comma is required to tell them apart
    // from a literal surrounded by parentheses.
    println!("One element tuple: {:?}", (5u32,));
    println!("Just an integer: {:?}", (5u32));

    // Tuples can be destructured to create bindings.
    let tuple = (1, "hello", 4.5, true);

    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix);
    println!("{}", matrix.0);

}

fn return_tuple() -> (u32, u64) {
    return (23, 45);
}

fn analyze_slice(slice: &[i32]) {
    println!("First element of the slice: {}", slice[0]);
    println!("The slice has {} elements", slice.len());
}

pub fn execute_array() {

    let fresh_array: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
    println!("fresh_array: {:?}", fresh_array);
    println!("first value in fresh array: {}", fresh_array.get(3).expect("fresh arrsy"));

    let ys: [i32; 500] = [0; 500];

    // Indexing starts at 0.
    println!("First element of the array: {}", fresh_array[0]);
    println!("Second element of the array: {}", fresh_array[1]);

    // `len` returns the count of elements in the array.
    println!("Number of elements in array: {}", fresh_array.len());

    // Arrays are stack allocated.
    println!("Array occupies {} bytes", mem::size_of_val(&fresh_array));

    println!("Borrow the whole array as a slice.");
    analyze_slice(&fresh_array);

    // Slices can point to a section of an array.
    // They are of the form [starting_index..ending_index].
    // `starting_index` is the first position in the slice.
    // `ending_index` is one more than the last position in the slice.
    println!("Borrow a section of the array as a slice.");
    analyze_slice(&ys[1 .. 4]);

    let empty_slice: [i32; 0] = [];
    assert_eq!(&empty_slice, &[]);
    assert_eq!(&empty_slice, &[][..]);

    for i in 0..fresh_array.len() + 1 { // Oops, one element too far!
        match fresh_array.get(i) {
            Some(xval) => println!("{}: {}", i, xval),
            None => println!("Slow down! {} is too far!", i),
        }
    }

}