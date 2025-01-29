use super::animal::Animal;
use std::fmt;

pub struct Tortue {
    nom: String
}

impl Animal for Tortue {
    fn creer(nom: String) -> Self {
        Tortue { nom }
    }

    fn emettre_son(&self) -> String {
        "stridulation".to_string()
    }

    fn obtenir_nom(&self) -> String {
        self.nom.clone()
    }
}

impl fmt::Display for Tortue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} ::: {})", self.obtenir_nom(), self.emettre_son()) 
    }
}
