// Structs in Rust
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// Function to create new User
fn create_user(username: String, email: String) -> User {
    User {
        username,
        email,
        sign_in_count: 0,
        active: false,
    }
}

// Function to get the length of a string
fn get_length_str(s: &str) -> usize {
    s.len()
}

fn main() {
    println!("Hello, world!");

    // Test get_length_str -> get the length of a string
    let s = "Hello, world!";
    println!("The length of '{}' is {}.", s, get_length_str(s));

    // Test array in Rust
    let arr = [1, 2, 3, 4, 5];
    println!("The first element of the array is {}.", arr[0]);

    // Test vector in Rust
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    println!("The first element of the vector is {}.", vec[0]);

    // Test struct in Rust
    let user = User {
        username: String::from("John Doe"),
        email: String::from("abc@gmail.com"),
        sign_in_count: 1,
        active: true,
    };
    println!("The username of the user is {}.", user.username);
    println!("The email of the user is {}", user.email);
    println!("The sign in count of the user is {}.", user.sign_in_count);
    println!(
        "The active status of the user is {}.",
        if user.active { "active" } else { "inactive" }
    );

    // Test create_user function
    let other_user = create_user(String::from("Jane Doe"), String::from("gg@gmail.com"));
    println!("The username of the other user is {}.", other_user.username);
    println!("The email of the other user is {}", other_user.email);

    // Print one struct
    println!("{:?}", user);
    // Print pretty struct
    println!("{:#?}", user);
}
