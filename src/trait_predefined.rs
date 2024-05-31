// On voit ici les traits "prédéfinis", cad les traits déjà implémentés pour les structures classiques
// Il y en a quatre :
//  - Copy              Copie implicite grâce à l'opérateur d'affectation (copie bit à bit : ne peut pas être surchargé)
//  - Clone             Copie explicite avec la méthode Clone()
//  - Debug             Permet l'affichage rapide de l'instance avec son "fmt" (avec :? dans un print!)
//  - PartialEq         Permet la comparaison d'objet avec == et !=

#[derive(Copy, Clone, Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32
}

pub fn using_traits() {
    let p1 = Point { x: 10, y: 20 };

    // Trait Clone
    let p2 = p1.clone();
    println!("{}, {}", p2.x, p2.y);
    
    // Trait Copy
    let p3 = p2;
    println!("{}, {}", p3.x, p3.y);

    // Trait Debug
    println!("{:?}", p3);

    // Trait PartialEq
    if p3 == p1 {
        println!("PartialEq : {}", p3 == p1);
    }

    let vect = vec![1, 2];
    for el in &vect {

    }
    let mut iter = vect.iter();

    while let Some(element) = iter.next() {

    }
}
