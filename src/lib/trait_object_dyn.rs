// Par défaut, une surchage produite par une fonction générique est dite surchage statique
// En effet, chaque utilisation de la fonction générique avec un type encore non utilisé va produire une implémentation par copie brute
//      de la fonction au moment de la compilation

// Il est donc possible de faire de la surcharge "dynamique" (au runtime)
// Un objet-trait est un type qui inclut un ensemble de traits. On manipule ici un pointeur vers un trait

use std::fmt::Display;

struct S1 { 
    valeur: i64 
} 
struct S2 { 
    valeur: f64 
} 

struct S3 { 
    valeur:  String 
} 

fn fn_surcharge_static<T:Display>(var_struct: T) {
    println!("{}", var_struct);
}

pub fn using_trait_object() {
    let s1 = S1{valeur : 42}; 
    let s2 = S2{valeur : 3.30}; 
    let s3 = S3{valeur : "Bonjour tout le monde.".to_string()}; 
    fn_surcharge_static(s1.valeur); 
    fn_surcharge_static(s2.valeur); 
    fn_surcharge_static(s3.valeur);

    fn_surcharge_dynamique(&s1);
    fn_surcharge_dynamique(&s2);
}

trait Affichage { 
    fn afficher_valeur(&self); 
}

impl Affichage for S1 { 
    fn afficher_valeur(&self) { 
        println!("Entier {:?}", self.valeur); 
    } 
} 

impl Affichage for S2 { 
    fn afficher_valeur(&self) { 
        println!("Flottant {:?}", self.valeur); 
    } 
} 

impl Affichage for S3 { 
    fn afficher_valeur(&self) { 
        println!("String {:?}", self.valeur); 
    } 
} 

fn fn_surcharge_dynamique(objet_implementant_trait: 
    &(dyn Affichage + 'static)) { 
     
       objet_implementant_trait.suivi_appel_surcharge_dynamique(); 
       objet_implementant_trait.afficher_valeur(); 
    } 

impl dyn Affichage + 'static { 
    fn suivi_appel_surcharge_dynamique(&self) { 
        println!("Ici on utilise un objet-trait."); 
    } 
}