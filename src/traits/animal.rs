// Trait
pub trait Animal {
    fn creer(nom: String) -> Self;

    fn emettre_son(&self) -> String;
    fn obtenir_nom(&self) -> String;

    fn afficher(&self) {
        println!("{} : {}", self.obtenir_nom(), self.emettre_son());
    }
}
