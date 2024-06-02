extern crate regex;
use regex::Regex;

// un char se code sur 4 octets (32 bits)
// str (ou &str, &'static str) est le type primitif des strings. Il s'agit d'une séquence d'octet non mutable de chars UTF8
//      taille non connue. C'est le "string slice" comme on l'appelle
// String est une structure composée uniquement d'un Vec<u8>, donc taille sans limite, avec plein de méthodes disponibles

pub fn using_chars() {
    let car = 'a';

    println!("Caractère '{}'", car);
    println!("Caractère numérique ? {}", car.is_numeric());
    println!("Caractère alphabétique ? {}", car.is_alphabetic());
    println!("Caractère alphanumérique ? {}", car.is_alphanumeric());
    println!("Caractère espace ? {}", car.is_whitespace());
    println!("Caractère lowercase ? {}", car.is_lowercase());

    let upper = car.to_uppercase();     //Cela ne crée pas de Char mais un Iterator, car un encodage peut transformer un char en plusieurs
    let lower = car.to_lowercase();

    let intu_32 = car as u32;

    let car_2 = char::from_u32(intu_32);
}

pub fn using_strings() {
    // str
    let chaine = "hello";
    let sliced = &chaine[2..];
    chaine[2..].contains('o');
    chaine.split('a');
    chaine.trim();

    // String
    let mut string_1 = String::new();
    string_1.push('a');

    let mut string_2 = String::from("test");
    println!("string : {}", string_2);
    println!("length : {}", string_2.len());
    println!("capacity : {}", string_2.capacity());

    println!("Ajout d'un caractère.");
    string_2.push('0');
    println!("string : {}", string_2);
    println!("length : {}", string_2.len());
    println!("capacity : {}", string_2.capacity()); //On voit ici que quand on ajoute un caractère à la chaine qui dépasse la capacité
                                                    // Celle-ci se voit doublé
    string_2.pop();
    
    let string_3 = "string 3".to_string();
    string_3.is_empty();

    let mut string_4 = String::with_capacity(8);
    string_4.push_str(chaine);               //Ajout d'une chaîne
    string_4.push_str(&string_2[..]);   //Ajout d'un slice

    string_4.insert_str(1, &string_2[1..]);

    string_4 += "test";     // Cela fonctionne car le trait AddAssign existe sur String, qui fait simplement un push_str()

    string_4.clear(); //Supprime le contenu, donc len sera à 0 mais la capacity reste identique

    string_4 = "test".to_string();
    string_4.remove(3);

    // Une autre méthode pour extraire des caractères consiste en créer un itérateur avec "drain" et de collecter le résultat avec "collect" :
    let drain = string_4.drain(1..3);
    let extraction: String = drain.collect::<String>();

    if let Some(x) = extraction.find("es") { 
        println!("find location : {}", x);
    }

    extraction.contains("es");
    extraction.starts_with("es");
    extraction.ends_with("es");

    let replaced = extraction.replace("es", "se");
}

pub fn using_regex() {
    let date_exemple = "2022-01-17";

    let rex = Regex::new(r"(?x) 
    (?P<annee>\d{4}) 
    - 
    (?P<mois>\d{2}) 
    - 
    (?P<jour>\d{2}) 
    ").unwrap(); 

    let parsing_date = rex.captures(date_exemple).unwrap(); 
 
    println!("Année : {}", &parsing_date["annee"]); 
    println!("Mois : {}", &parsing_date["mois"]); 
    println!("Jour : {}", &parsing_date["jour"]); 
}