fn main() {
    println!("Hello, world!");
}

fn destroy(val: String) {
    println!("{}", val);
}

//References

fn references() {
    let mut original_value: i32 = 20;

    //Borrow the value of orginial_value
    let x = &original_value; // & is a reference

    original_value = String::from("Josh");

    let x = &original_value;

    println!("{}", x); //Will print now because we re borrowing again

    //Object is "destroyed" when it leaves scope
    destroy(original_value);
    println!("{}", x); //Will not print
}

//Dereferencing
fn dereferencing() {
    let a = 1;
    let b = &a;

    assert_eq!(1, a);
    assert_eq!(1, b); // *b = a
}

//Static
fn statics() {
    let x: &'static str = "Dave";
    let y = &X;
    destroy(x.to_string()); // x is not being destroyed, it's static
    println!("{}", y);

}

// usize
fn usize_example() {
    let x: usize = 2; // Depends on OS architecture (x32, x64)
}