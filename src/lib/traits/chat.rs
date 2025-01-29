use super::animal::Animal;

// Structure Chat
pub struct Chat {
    nom: String
}

// Implémentation de la struct Chat
impl Chat {
    pub fn dormir(&self) {
        println!("Comme tous les chats, je dors");
    }
}

// Implémentation du trait Animal pour Chat
impl Animal for Chat { 
 
    fn creer(nom: String) -> Self { 
       Chat { nom } 
    } 
  
    fn emettre_son(&self) -> String { 
       "miaulement".to_string() 
    } 
  
    fn obtenir_nom(&self) -> String { 
        self.nom.clone()
    }
} 