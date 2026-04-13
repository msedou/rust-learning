use std::io;

pub fn read_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Erreur");
    return input.trim().to_string();
}
