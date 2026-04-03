// fn main() {
//     let name = 5;
//     let age = 25;
//     println!("Hello, {} year old {}", name, age);
// }

// fn main() {
//     let x: i32 = 10;
//     let y: f64 = 3.14;
//     let is_active: bool = true;
//     let letter: char = 'A';

//     println!("x is {}", x);
//     println!("y is {}", y);
//     println!("is_active is {}", is_active);
//     println!("letter is {}", letter);
// }

// fn main() {
//     let mut count: i32 = 0;
//     count = count + 1;
//     println!("Count: {}", count);
// }

// fn main() {
//     let name: &str = "Seydou";
//     let age: i32 = 25;
//     let is_active: bool = true;

//     println!("Name:{}", name);
//     println!("Age:{}", age);
//     println!("Active:{}", is_active);
// }
// struct Utilisateur {
//     pseudo: String,
//     email: String,
//     number_of_connexion: u64,
//     is_active: bool,
// }

// fn main() {
//     let user1 = Utilisateur {
//         pseudo: String::from("Seydou"),
//         email: String::from("msedou@example.com"),
//         is_active: true,
//         number_of_connexion: 1,
//     };

//     println!("Pseudo {}", user1.pseudo);
// }

// struct Couleur(i32, i32, i32);
// struct Point(i32, i32, i32);

// fn main() {
//     let black_color: Couleur = Couleur(0, 0, 0);
//     let with_color: Couleur = Couleur(255, 255, 255);
//     let black_point: Point = Point(0, 0, 0);
//     let with_point: Point = Point(255, 255, 255);
// }

// use std::io::Chain;

// struct TriApha; //structure vide
// trait StrategieDeTri {
//     fn traiter(&self, list: &mut Vec<String>);
// }

// impl StrategieDeTri for TriApha {
//     fn traiter(&self, list: &mut Vec<String>) {
//         list.sort(); //logique de trie alphabetique
//     }
// }

struct User {
    name: String,
    age: i32,
    active: bool,
}

fn main() {
    let user1: User = User {
        name: String::from("Seydou"),
        age: i32::from(25),
        active: bool::from(true),
    };
    // println!("Name {}", user1.name);
    user1.display();
}

impl User {
    fn display(&self) {
        println!("Name {}, age {}", self.age, self.name);
    }
}
