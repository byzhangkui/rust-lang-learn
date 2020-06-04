fn main() {
    let s = String::from("hello world!");

    let hello = &s[0..5];// hello
    let world = &s[6..11];// world

    println!("{}", hello);
    println!("{}", world);

    let s = String::from("hello");
    let slice = &s[0..2];
    let slice = &s[..2]; // if start with zero, can drop zero

    let slice = &s[3..s.len()];
    let slice = &s[3..]; // if includes the last byte of String, can drop the trailing number.

    let slice = &s[0..s.len()];
    let slice = &s[..]; // drop both values, take a slice of the entire string
}
