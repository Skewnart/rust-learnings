// Trait d'itération : Iterator, IntoIterator
// Trait Drop : s'execute à la destruction d'une valeur
// Deref, DerefMut : relatif au pointeurs intelligents pour le déréférencement
// AsRef, AsMut, Borrow et BorrowMut, traits d'emprunt de référence
// From, Into pour les conversions implicites
// ToOwned qui permet de convertir une référence en valeur

mod chien;
mod chat;
mod animal;
mod tortue;
mod poisson;

use animal::Animal;
use chien::Chien;
use chat::Chat;
use tortue::Tortue;

use std::fmt::{self, Display};

pub fn using_traits() {
    let chien = Chien::creer("Cabo".to_string());
    println!("{}", chien.obtenir_nom());
    chien.afficher();

    let kiks = Chat::creer("Kiks".to_string()); 
    println!("{}", kiks.obtenir_nom()); 
    kiks.dormir(); 
    kiks.afficher();

    afficher_trait(&chien);
    afficher_trait(&kiks);
}

// Trait en paramètre
//Cette méthode (sucre syntaxique) :
fn afficher_trait(animal: &impl Animal) {
    animal.afficher();
}
//Equivaut à : 
fn afficher_trait_2<T: Animal>(animal: &T) {
    animal.afficher();
}

// ------------------
pub fn using_traits_2() {
    let tortue = Tortue::creer("tort".to_string());
    println!("{}", tortue);
}

// Ceci :
fn display(element: &(impl Animal + fmt::Display)) {
    println!("{}", element);
}

// Peut bien sûr s'écrire ainsi :
fn display_2<T: Animal + fmt::Display>(element: T){
    println!("{}", element);
}

//###### Autre exemple
// Ceci :
fn other_display<T: Animal + Display, U: Animal + fmt::Debug>(t: &T, u: &U) -> String {
    "".to_string()
}
// Peut s'écrire avec la clause "where" :
fn other_display_2<T, U>(t: &T, u: &U) -> String
    where T : Animal + Display,
        U : Animal + fmt::Debug  {
    "".to_string()
}
