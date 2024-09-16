mod provider;
mod user;

use provider::_Provider;
use user::User;

// Structs in Rust
#[derive(Debug)]
struct UserInFile {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

impl UserInFile {
    fn get_username(&self) -> &String {
        &self.username
    }

    fn get_email(&self) -> &String {
        &self.email
    }

    fn get_sign_in_count(&self) -> u64 {
        self.sign_in_count
    }

    fn is_active(&self) -> bool {
        self.active
    }

    fn is_active_to_string(&self) -> String {
        if self.active {
            String::from("active")
        } else {
            String::from("inactive")
        }
    }
}

// Function to create new User
fn create_user(username: String, email: String) -> UserInFile {
    UserInFile {
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
    let user = UserInFile {
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

    // Test User methods
    println!("Test User methods");
    println!("The username of the user is {}.", user.get_username());
    println!("The email of the user is {}", user.get_email());
    println!(
        "The sign in count of the user is {}.",
        user.get_sign_in_count()
    );
    println!(
        "The active status of the user is {}.",
        if user.is_active() {
            "active"
        } else {
            "inactive"
        }
    );
    println!(
        "The active status of the user is {}.",
        user.is_active_to_string()
    );

    // print one string for 50 times
    println!("{}", "-".repeat(50));

    // Test User struct other file
    let user_other_file = User::new(
        100,
        String::from("John"),
        String::from("hypl1001@gmail.com"),
    );

    println!("The id of the user is {}.", user_other_file.get_id());
    println!(
        "The username of the user is {}.",
        user_other_file.get_name()
    );
    println!("The email of the user is {}.", user_other_file.get_email());
    println!(
        "The sign in count of the user is {}.",
        user_other_file.get_id()
    );
    println!("The data user to string is {}", user_other_file.to_string());

    // Test with enum and struct in provider.rs
    let provider_name = provider::_Provider {
        kind: provider::ProviderKind::Name(String::from("John Doe")),
        info: String::from("This is the name of the provider."),
    };
    println!("The kind of the provider is {:#?}", provider_name.kind);
}
