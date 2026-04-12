use std::fmt;

// 1. Gestion des erreurs personnalisée
#[derive(Debug)]
enum ParseError {
    EmptyInput,
    MissingSeparator,
}

// Implémentation pour l'affichage (obligatoire pour Error)
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::EmptyInput => write!(f, "L'entrée est vide"),
            Self::MissingSeparator => write!(f, "Séparateur ':' manquant"),
        }
    }
}

// 2. Structure avec Lifetime
// 'a indique que Message ne peut pas survivre au buffer source
struct Message<'a> {
    user: &'a str,
    content: &'a str,
}

// 3. Logique combinée
fn parse_message<'a>(input: &'a str) -> Result<Message<'a>, ParseError> {
    if input.is_empty() {
        return Err(ParseError::EmptyInput);
    }

    // On cherche l'index du séparateur
    let split_idx = input.find(':').ok_or(ParseError::MissingSeparator)?;

    // On découpe sans copier (slices)
    let user = &input[..split_idx];
    let content = &input[split_idx + 1..].trim();

    Ok(Message { user, content })
}

fn main() -> Result<(), ParseError> {
    let raw_data = String::from("Alice:Salut Rust !");
    
    // Le parseur utilise une référence à raw_data
    let msg = parse_message(&raw_data)?;

    println!("Utilisateur: {}, Message: {}", msg.user, msg.content);
    
    // Si on supprimait `raw_data` ici, `msg` ne compilerait plus.
    // C'est la garantie de sécurité des lifetimes.
    
    Ok(())
}
