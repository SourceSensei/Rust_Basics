fn main() {
    println!("Hello, world!");
}

//Can't overwrite an immutable var
fn immutable() {
    let x = 5;
    x = 6;
}

//Must overwrite with "let"
fn overwrite() {
    let x = 5;
    let x = 6;
}

//Can't mutate a var type
fn mutate_type() {
    let some_string = "donut";
    some_string = some_string.len();
}

//Must mutate with "let"
fn overwrite_type() {
    let some_string = "donut";
    let some_string = some_string.len();
}

//Declare data types
fn declare() {
    let x: u32 = 20;
}

//Integers
fn integers() {
    let signed_int: i64 = 34;
    let unsigned_int: u32 = 11;
}

//Boleans
fn booleans() {
    let t: bool = true;
    let f: bool = false;
}

//Strings
fn strings() {
    let some_slice: &str = "donut"; //Slice
    let some_string: String = String::from("donut"); //String
}

// A string slice is a reference to a piece of text stored somewhere else (typically in memory).
// It is a borrowed view of a string and does not own the data it references.
// String slices are used when you want to work with text but don't need to modify or own it.
// Commonly used for function parameters to read text without taking ownership.


// A String is a type that owns its text data.
// It is mutable, resizable, and can be modified or extended.
// Strings are used when you need to create, modify, or own the text data.


//Constants
const SOME_CONSTANT: u32 = 20;