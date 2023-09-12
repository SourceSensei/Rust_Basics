fn main() {
    println!("Hello, world!");
}

//Struct 
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

//Initialize
fn initialize_struct() {
    let user1 = User {
        username: String::from("someuser123"),
        email: String::from("some@example.com"),
        sign_in_count: 1,
        active: true,
    };
}

// When params have the same name you can just pass them
fn initialize_struct_simple(email: String, username: String) {
    let user1 = User {
        email,
        username,
        sign_in_count: 1,
        active: true,
    };
}

//Updating
fn update_struct() {
    let user1 = User {
        email: String::from("some@example.com"),
        username: String::from("someotheruser123"),
        sign_in_count: 1,
        active: true,
    };
    user1.email = String::from("new@example.com");
    //Inherit the rest of the values from one user
    let user2 = User {
        email: String::from("john@example.com"),
        username: String::from("john123"),
        ..user1
    };
}

//Methods
struct Person {
    first_name: String,
    last_name: String,

}

impl Person {
    // Method (uses "&self")
    fn details(&self) -> String {
        String::from(&self.last_name)
    }
    //Associated Function (does not use "&self")
    fn more_details() -> String {
        String::from("pizza")
    }
}

fn example() {
    let george = Person {
        first_name: String::from("George"),
        last_name: String::from("Lopez"),
    };
    println!("{}", george.details());
    println!("{}", Person::more_details());

}