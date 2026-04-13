use crate::domain::user::User;
pub fn create_user(name: String, age: u32) -> User {
    User { name, age }
}

pub fn display_user(user: &User) {
    println!("Nom: {}, Age: {}", user.name, user.age)
}
