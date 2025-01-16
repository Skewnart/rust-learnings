// Pour des raisons de capacité d'un vecteur, si on connaît d'emblée la taille qu'il doit avoir, faire en sorte de bien le dimensionner tout de suite

use std::{arch::x86_64, array, vec};

pub fn using_vectors() {
    let vec_int = vec![1,2,3];
    display_type(&vec_int);

    let vec_empty: Vec<i64> = vec![];
    display_type(&vec_empty);

    let vec_str = vec!["Paris, Marseille, Antibes"];
    display_type(&vec_str);

    //reference
    let element_1 = &vec_str[0];

    for ii in 0..vec_int.len() {
        println!("{}", &vec_int[ii]);
    }

    let tranche = &vec_int[1..=2];
    println!("{}", tranche[0]);
    println!("{}", tranche[1]);

    let new_vector = tranche.to_vec();      //copie

    //Sans le &, on fait donc une copie (soit le trait Copy, soit Clone)
    let element_with_copy = new_vector[0];
    println!("element avec copie : {}", element_with_copy);
    let element_with_clone = new_vector[0].clone();     //Ne fait pas vraiment le clone puisque i32 n'implémente pas ce trait, mais le principe est là
    println!("element avec clone : {}", element_with_clone);

    //Méthode pour obtenir les éléments en Option plutôt que d'avoir des panics
    let el1 = vec_int.get(1);       //: Option<&i32>
    let el2 = vec_int.first();
    let el3 = vec_int.last();

    let mut array_mut = [5, 10, 15];
    let mut el_mut: Option<&mut i32> = array_mut.last_mut();

    //Si c'est pas grave que le programme panic, on peut unwrap une Option
    let mut opt = el_mut.unwrap();
    let opt2 = el3.unwrap_or(&0);
    *opt = 3;   //Opérateur de déréférencement

    println!("{:?}", array_mut);

    let mut vec_mutable = vec![1];
    vec_mutable.reserve(5);     //Regarde si on peut accueillir 5 éléments, sinon augmentation de capacité, avec les x2
    vec_mutable.reserve_exact(7);

    vec_mutable.push(1);        //Ajouter un élément
    vec_mutable.push(2);

    // !! Comme la capacity double quand elle n'est pas assez grande, attention si cette opération est fréquente ça va devenir lent.
    // Auquel cas utiliser une VecDequeue

    for i in &vec_mutable {
        println!("{}", i);
    }

    //Stocker plusieurs types : passer par un enum
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    other_methods();
}

fn other_methods() {
    let mut vector: Vec<i32> = (0..10).collect();
    let vector_2: Vec<i32> = (0..=5).collect();
    let mut vector_3: Vec<i32> = (0..=2).collect();
    
    vector.resize(15, 0);           //change la taille et comble les nouvelles valeurs avec 'value'
    println!("{:?}", vector);
    vector.truncate(10);                       //tronque
    vector.extend(vector_2);                  //ajoute le 2e vecteur au premier
    vector.split_off(9);                        //Fait comme le truncate mais renvoie une copie du slice retiré
    vector.append(&mut vector_3);                   //Comme extend mais qui déplace le contenu du vector dans le premier. Donc mut obligatoire
    vector.dedup();                                 //Supprime les valeurs en double
    vector.drain(1..2);                       //Enlève les valeurs de la plage du vecteur et crée un itérateur sur ce qui a été retiré
    vector.retain(|x| x % 2 == 0);            //On retire toutes les valeurs qui ne matchent pas le test

    vector.binary_search(&5);                       //Retour un Result (Ok ou Err)
    //vector.binary_search_by() avec une closure
}

fn display_type<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}