/*
    Les trois règles d'or du ownership :
    - Tout objet a un propriétaire (owner)
    - Un seul propriétaire à la fois, peut céder sa propriété à un autre
    - Si le scope du propriétaire est terminé, l'objet est détruit

    - Tout passage de valeur à une fonction passe par proprété (move) et non par copie ou référence
    - Si l'objet possède le Trait Copy, le comportement par défaut sera alors la copie.

    - Stack : Tout objet de taille connue
    - Heap : Taille inconnue
        ex : i32 -> stack, Vector<i32> -> heap avec une référence (+ length + capacity) dans la stack

    &mut est exclusive, une et une seule ref mutable possible à la fois
*/

pub fn using_ownership() {

    let s1 = String::from("Hello");
    let s2 = s1;         // s1 est déplacé dans s2 (move), s1 devient inutilisable
    let s3 = s2.clone(); // s2 est cloné puis stocké dans s3, s2 est utilisable

    let a = 5;
    let b = a;      //Ici, a(i32) est copié (Trait Copy implémenté), a est toujours utilisable

    let string = String::from("Hello");
    takes_ownership(string); // string devient inutilisable

    let string2 = String::from("Hello");
    string2 = takes_and_returns_ownership(string2); //string2 perd une première fois l'owernship puis le récupère à la fin de la fonction

    does_not_take_ownership(&string2);

    let mut mut_s = String::from("Hello");
    mutable_reference(&mut mut_s);

    //SLICE
    // Un string slice (&str) est une partie d'un String
    let string = String::from("Hello");
    let slice = &string[0..2];
    //un slice est forcément une référence.
}

fn takes_ownership(s : String) {
    println!("s : {s}");
}

fn takes_and_returns_ownership(s : String) -> String {
    println!("s : {s}");
    s
}

fn does_not_take_ownership(s : &String) {
    println!("s : {s}");
}

fn mutable_reference(s : &mut String) {
    s.push_str(" added");
}

fn dangle() -> &String {
    let s = String::from("Hello");
    &s // Retourne une référence à s

    //Ce qui est une très mauvaise pratique, car s est détruit après cette ligne (son scope),
    //le scope étant plus petit que l'endroit où il envoyé, ca impliquerait d'avoir un lifetime static proposé par Rust (seconde mauvaise pratique, car c'est sûrement pas ce qu'on veut)
    // voir les règles d'élision de lifetimes 
}