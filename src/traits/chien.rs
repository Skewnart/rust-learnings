use super::animal::Animal;

// Structure Chien
pub struct Chien {
    nom: String
}

// Implémentation du trait Animal pour Chien
impl Animal for Chien {
    fn creer(nom: String) -> Self { 
        Chien { nom } 
    } 
   
    fn emettre_son(&self) -> String { 
        "aboiement".to_string()
    } 
   
    fn obtenir_nom(&self) -> String { 
        self.nom.clone() 
    } 
   
    // Surcharge de l'implémentation disponible dans le trait. 
    fn afficher(&self) { 
        println!("Moi le chien {}, j'émets un {}.",
            self.obtenir_nom(), self.emettre_son()); 
    } 
} 