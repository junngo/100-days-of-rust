struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(username: String, email: String) -> User {
    User {
        username,
        email,
        sign_in_count: 1,
        active: true,
    }
}

fn main() {
    println!("Test is for the Struct");
    // 1. Basic for creating the user
    let user1 = User {
        email: String::from("test_email@gmail.com"),
        username: "username123".to_string(),
        sign_in_count: 1,
        active: true,
    };
    println!("user1 name: {0}", user1.username);

    // 2. To create the user by function
    let user2 = build_user("user2".to_string(), "user2_email@gmail.com".to_string());
    println!("user2 name: {0}", user2.username);

    // 3. Creating Instances from Other Instances with Struct Update Syntax (Field Init Shorthand)
    let user3 = User{
        username: "user3".to_string(),
        email: "user3_email@gmail.com".to_string(),
        ..user1
    };
    println!("user3 name: {0}", user3.username);
}
