struct User {
    // we used the owned String type rather than the &str string slice type. This is a deliberate choice because we want instances of this struct to own all of its data and for that data to be valid for as long as the entire struct is valid
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}


fn main() {
    
    //immutable
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    //to change any value of fields, the entire instance must be mutable
    let mut user2 = User {
        email: String::from("user2@example.com"),
        username: String::from("someusername456"),
        active: true,
        sign_in_count: 1,
    };

    user2.email = String::from("anotheremail@example.com");

    // Creating Instances From Other Instances With Struct Update Syntax

    // - without update syntax
    let user3 = User {
        email: String::from("user3@example.com"),
        username: String::from("someusername789"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };

    // - update syntax

    let user4 = User {
        email: String::from("user4@example.com"),
        username: String::from("anotherusername789"),
        ..user1  // The syntax .. specifies that the remaining fields not explicitly set should have the same value as the fields in the given instance.
    };



    // Using Tuple Structs without Named Fields to Create Different Types
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    // black and origin values are different types, because they’re instances of different tuple structs. Each struct you define is its own type, even though the fields within the struct have the same types
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

fn build_user(email: String, username: String) -> User {
    User {
        // field init shorthand syntax, same as js
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}