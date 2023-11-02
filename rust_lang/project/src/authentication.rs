mod authentication {
    pub struct User {
        username: String, 
        password_hash: u64,
    }

    impl User {
        pub fn new(username: &str, password: &str) -> User {
            User {
                username: username.to_String(),
                password_hash: hash_password(password),
            }
        }
    }

    fn hash_password(password: &str) -> {/* */}
}

fn main() {
    let user = authentication::User::new("name", "secret");
}