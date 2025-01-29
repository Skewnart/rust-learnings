// Traits "prédéfinis" -> les traits dont une implémentation par défaut existe pour les structures classiques :
// - Eq, PartialEq, Ord et PartialOrd, comparaison d'objets
// - Clone, qui permet de créer T depuis une référence &T.
// - Copy, copie bit à bit -> passage de la sémantique move à la sémantique copy. (ne peut pas être surchargé)
// - Hash, qui permet de produire un hachage depuis &T.
// - Default, qui permet de créer une instance par défaut du type considéré.
// - Debug, qui permet de formater une sortie sur {:?}.

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
