use super::animal::Animal;

// Exemple de sous trait :
pub trait Poisson : Animal {
    fn nager() {

    }
}

pub struct PoissonRouge {
    nom: String
}

impl Poisson for PoissonRouge {
    fn nager() {

    }
}

impl Animal for PoissonRouge {
    fn creer(nom: String) -> Self {
        PoissonRouge { nom }
    }

    fn emettre_son(&self) -> String {
        "Boup boup".to_string()
    }

    fn obtenir_nom(&self) -> String {
        self.nom.clone()
    }
}