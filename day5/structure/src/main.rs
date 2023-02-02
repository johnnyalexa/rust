fn main() {
    println!("Hello, world!");


    let user1 = User {
        email: String::from("test@gmail.com"),
        username: String::from("Vasile"),
        active: true,
        sign_in_count: 2,
    };

    println!("The user {} has email {}", user1.username, user1.email);

    let user2 = build_user(String::from("iont@gmail.com"), String::from("Ion"));
    println!("The user {} has email {}", user2.username, user2.email);

    let user3 = User {
        email: String::from("gica@gmail.com"),
        ..user1
    };

    println!("The user {} has email {}", user3.username, user3.email);
    
    let black = Color(0,0,0);
    let origin = Point(0,0,0);

    let subject = AlwaysEqual;

    let user4 = NewUser {
        email: "someemail@gmail.com",
        username: "gabriel",
        active: true,
        sign_in_count: 4,
    }

    println!("The user {} has email {}", user4.username, user4.email);
}


struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}


struct NewUser {
    active: bool,
    username: &str,
    email: &str,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

struct Color (i32, i32, i32);
struct Point (i32, i32, i32);

struct AlwaysEqual;