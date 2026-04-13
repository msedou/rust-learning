use super::model::User;
// fn get_user() -> String {
//     String::from("Seydou")
// }
pub fn create_user(name: String, age: u32) -> User {
    User { name, age }
}

pub fn display_user(user: &User) {
    println!("Name {} , Age {}", user.name, user.age);
}
