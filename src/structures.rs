// 2 types :
// - Structure
// - Tuple

//Remove warning non snake case dans les propriétés
#![allow(non_snake_case)]

//struct ici
struct Person {
    name : String,
    age : i8,
    drivingLicense: bool
}

//struct tuple ici
struct PersonTuple (String, i8, bool);

pub fn using_structures() {
    let person = Person { name : String::from("Jarod"), age : 21, drivingLicense : true };

    println!("Name : {}", person.name);
    println!("Age : {}", person.age);

    let mut person2 = Person { name : String::from("Leto"), age : 25, drivingLicense : false };
    person2.drivingLicense = true;              //First method
    person2 = Person { age : 31, ..person2};    //Second method (range operator)


    let persontuple = PersonTuple(String::from("John"), 23, true);
    println!("{}", persontuple.0);
}

//Pourquoi utiliser une structure tuple 
// -> Souvent vu pour définir des unités : (C'est mieux que d'utiliser l'unité dans le nom)

//Exemple :

struct Meter(f32);

pub fn using_structure_tuple() {
    let distance = Meter(12.5);

    //Par destructuration (destructuring) :
    let Meter(m) = distance; 
    println!("{}", m);

    //Par accès direct, moins lisible :
    let m2 = distance.0;
    println!("{}", m2);
}
