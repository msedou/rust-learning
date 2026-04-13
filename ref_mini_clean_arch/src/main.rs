// Déclaration obligatoire des modules racines
mod application;
mod domain;
mod interface;

// Import des éléments dont on a besoin
use application::user_service::{create_user, display_user};
use interface::cli::read_input;

fn main() {
    let name = read_input("Enter name :");
    let age_input = read_input("Enter age :");
    let age: u32 = age_input.parse().expect("Invalid number");
    if age > 0 {
        let user = create_user(name, age);
        display_user(&user);
    } else {
        println!("Erreur ! age ne peut etre inferieur a zero !");
    }
}
